use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use git2::Repository;

use super::schema::{DocEntry, DocStatus, DocsIndex, DocsMetadata, SkillEntry, SkillsIndex};

pub fn generate_skills_index(path: &Path, hub_id: &str) -> Result<SkillsIndex> {
    let repo = Repository::discover(path)?;
    let git_url = remote_url(&repo);
    let mut skills = Vec::new();

    for entry in std::fs::read_dir(path)?.filter_map(|e| e.ok()) {
        let skill_dir = entry.path();
        if !skill_dir.is_dir() {
            continue;
        }
        let skill_md = skill_dir.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&skill_md)?;
        let fm = parse_frontmatter(&content)?;

        let slug = skill_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let rel_path = skill_dir.strip_prefix(path)?.display().to_string();
        let commit = last_commit_hash(&repo, &skill_md);
        let has_lifecycle = skill_dir.join("lifecycle.yaml").exists();

        skills.push(SkillEntry {
            slug,
            name: fm_str(&fm, "name"),
            description: fm_str(&fm, "description"),
            version: fm
                .get("metadata")
                .and_then(|m| m.get("version"))
                .and_then(|v| v.as_str())
                .unwrap_or("0.1.0")
                .to_string(),
            compatibility: fm
                .get("compatibility")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            license: fm
                .get("license")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            git_url: git_url.clone(),
            path: rel_path,
            commit,
            has_lifecycle,
        });
    }

    Ok(SkillsIndex {
        hub_id: hub_id.to_string(),
        generated_at: Utc::now().to_rfc3339(),
        skills,
    })
}

pub fn generate_docs_index(path: &Path) -> Result<DocsIndex> {
    let repo = Repository::discover(path)?;
    let repo_commit = head_commit_hash(&repo);
    let mut entries = Vec::new();

    for file in glob_md_files(path) {
        let content = std::fs::read_to_string(&file)?;
        if !content.starts_with("---") {
            continue;
        }
        let fm = match parse_frontmatter(&content) {
            Ok(fm) => fm,
            Err(_) => continue,
        };

        let rel_path = file.strip_prefix(path)?.display().to_string();
        let commit_hash = last_commit_hash(&repo, &file);
        let read_when: Vec<String> = fm
            .get("read_when")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let status = match fm.get("status").and_then(|v| v.as_str()) {
            Some("active") => DocStatus::Active,
            Some("deprecated") => DocStatus::Deprecated,
            _ => DocStatus::Draft,
        };

        entries.push(DocEntry {
            title: fm_str(&fm, "title"),
            summary: fm_str(&fm, "summary"),
            path: rel_path,
            commit_hash,
            last_updated: fm_str(&fm, "last_updated"),
            status,
            read_when,
        });
    }

    let total = entries.len();
    Ok(DocsIndex {
        kind: "docs".into(),
        version: "1.0".into(),
        entries,
        metadata: DocsMetadata {
            generated_at: Utc::now().to_rfc3339(),
            commit_hash: repo_commit,
            total_entries: total,
        },
    })
}

fn parse_frontmatter(content: &str) -> Result<serde_yaml::Mapping> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    anyhow::ensure!(parts.len() >= 3, "invalid frontmatter");
    Ok(serde_yaml::from_str(parts[1])?)
}

fn fm_str(fm: &serde_yaml::Mapping, key: &str) -> String {
    fm.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn last_commit_hash(repo: &Repository, path: &Path) -> String {
    let mut revwalk = match repo.revwalk() {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let _ = revwalk.push_head();
    let _ = revwalk.set_sorting(git2::Sort::TIME);

    for oid in revwalk.filter_map(|r| r.ok()) {
        let commit = match repo.find_commit(oid) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let tree = match commit.tree() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let rel = match path.strip_prefix(repo.workdir().unwrap_or(path)) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if tree.get_path(rel).is_ok() {
            return oid.to_string();
        }
    }
    String::new()
}

fn head_commit_hash(repo: &Repository) -> String {
    repo.head()
        .ok()
        .and_then(|r| r.peel_to_commit().ok())
        .map(|c| c.id().to_string())
        .unwrap_or_default()
}

fn remote_url(repo: &Repository) -> String {
    repo.find_remote("origin")
        .ok()
        .and_then(|r| r.url().map(str::to_string))
        .unwrap_or_default()
}

fn glob_md_files(path: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() {
                files.extend(glob_md_files(&p));
            } else if p.extension().and_then(|e| e.to_str()) == Some("md") {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.to_uppercase() != "README.MD" {
                    files.push(p);
                }
            }
        }
    }
    files
}
