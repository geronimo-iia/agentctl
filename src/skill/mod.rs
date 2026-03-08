pub mod lifecycle;
pub mod lock;
pub mod vars;

use anyhow::{bail, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::hub::cache;
use lifecycle::{execute_lifecycle, sh_executor, Approver, LifecycleFile};
use lock::{LockEntry, LockFile};

pub fn skills_root(mode: Option<&str>) -> PathBuf {
    let base = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".agent");
    match mode {
        Some(m) => base.join(format!("skills-{m}")),
        None => base.join("skills"),
    }
}

pub fn install(
    cfg_path: &Path,
    lp: &Path,
    name: &str,
    hub_id: Option<&str>,
    mode: Option<&str>,
    quiet: bool,
    approver: Approver,
) -> Result<()> {
    let cfg = Config::load_from(cfg_path)?;

    // find hub entry
    let hub = match hub_id {
        Some(id) => cfg
            .skill_hubs
            .iter()
            .find(|h| h.id == id)
            .ok_or_else(|| anyhow::anyhow!("hub '{id}' not found"))?,
        None => cfg
            .skill_hubs
            .iter()
            .find(|h| h.enabled)
            .ok_or_else(|| anyhow::anyhow!("no enabled skill hub found"))?,
    };

    // load index from cache
    let index_json = cache::get(&hub.id, &hub.index_url, 6)?;
    let index: serde_json::Value = serde_json::from_str(&index_json)?;

    // find skill entry in index
    let skill_entry = index["skills"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("invalid index: missing skills array"))?
        .iter()
        .find(|s| s["slug"] == name || s["name"] == name)
        .ok_or_else(|| anyhow::anyhow!("skill '{name}' not found in hub '{}'", hub.id))?;

    let version = skill_entry["version"]
        .as_str()
        .unwrap_or("0.1.0")
        .to_string();
    let commit = skill_entry["commit_hash"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let skill_path_rel = skill_entry["path"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("missing path in skill entry"))?;

    // clone repo and copy skill dir
    let git_url = hub
        .git_url
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("hub '{}' has no git_url — cannot install", hub.id))?;

    let install_dir = skills_root(mode).join(name);
    clone_skill(git_url, &commit, skill_path_rel, &install_dir)?;

    // run lifecycle install
    let lifecycle_path = install_dir.join("lifecycle.yaml");
    if lifecycle_path.exists() {
        let yaml = std::fs::read_to_string(&lifecycle_path)?;
        let lf: LifecycleFile = lifecycle::parse(&yaml)?;
        let resolved_vars = vars::resolve(name, install_dir.to_str().unwrap_or(""), &lf.variables)?;
        execute_lifecycle(&lf.install, &resolved_vars, quiet, approver, sh_executor)?;
    }

    // write lock entry
    let mut lock = LockFile::load(lp)?;
    lock.insert(LockEntry {
        hub_id: hub.id.clone(),
        slug: name.to_string(),
        version,
        commit,
        installed_path: install_dir.to_string_lossy().to_string(),
        installed_at: Utc::now().to_rfc3339(),
    });
    lock.save(lp)?;

    println!("✓ Installed skill '{name}'");
    Ok(())
}

pub fn list(lp: &Path) -> Result<()> {
    let lock = LockFile::load(lp)?;
    if lock.skills.is_empty() {
        println!("No skills installed.");
        return Ok(());
    }
    for (key, entry) in &lock.skills {
        println!("  {} {} ({})", key, entry.version, entry.installed_path);
    }
    Ok(())
}

pub fn remove(lp: &Path, name: &str, hub_id: &str, quiet: bool, approver: Approver) -> Result<()> {
    let mut lock = LockFile::load(lp)?;
    let entry = lock
        .get(hub_id, name)
        .ok_or_else(|| anyhow::anyhow!("skill '{name}' not installed"))?
        .clone();

    let install_dir = PathBuf::from(&entry.installed_path);
    let lifecycle_path = install_dir.join("lifecycle.yaml");
    if lifecycle_path.exists() {
        let yaml = std::fs::read_to_string(&lifecycle_path)?;
        let lf: LifecycleFile = lifecycle::parse(&yaml)?;
        let resolved_vars = vars::resolve(name, &entry.installed_path, &lf.variables)?;
        execute_lifecycle(&lf.uninstall, &resolved_vars, quiet, approver, sh_executor)?;
    }

    if install_dir.exists() {
        std::fs::remove_dir_all(&install_dir)?;
    }
    lock.remove(hub_id, name);
    lock.save(lp)?;

    println!("✓ Removed skill '{name}'");
    Ok(())
}

pub fn update(
    cfg_path: &Path,
    lp: &Path,
    name: &str,
    hub_id: Option<&str>,
    quiet: bool,
    approver: Approver,
) -> Result<()> {
    let lock = LockFile::load(lp)?;

    // find existing entry to get hub_id if not provided
    let existing = if let Some(id) = hub_id {
        lock.get(id, name)
            .ok_or_else(|| anyhow::anyhow!("skill '{name}' not installed from hub '{id}'"))?
            .clone()
    } else {
        lock.skills
            .values()
            .find(|e| e.slug == name)
            .ok_or_else(|| anyhow::anyhow!("skill '{name}' not installed"))?
            .clone()
    };

    let cfg = Config::load_from(cfg_path)?;
    let hub = cfg
        .skill_hubs
        .iter()
        .find(|h| h.id == existing.hub_id)
        .ok_or_else(|| anyhow::anyhow!("hub '{}' not found", existing.hub_id))?;

    let index_json = cache::get(&hub.id, &hub.index_url, 0)?; // force refresh
    let index: serde_json::Value = serde_json::from_str(&index_json)?;

    let skill_entry = index["skills"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["slug"] == name || s["name"] == name)
        .ok_or_else(|| anyhow::anyhow!("skill '{name}' not found in hub"))?;

    let new_version = skill_entry["version"].as_str().unwrap_or("0.1.0");
    let new_commit = skill_entry["commit_hash"].as_str().unwrap_or("");

    if new_version == existing.version && new_commit == existing.commit {
        println!("Skill '{name}' is already up to date ({new_version}).");
        return Ok(());
    }

    let git_url = hub
        .git_url
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("hub '{}' has no git_url", hub.id))?;

    let skill_path_rel = skill_entry["path"].as_str().unwrap_or(name);
    let install_dir = PathBuf::from(&existing.installed_path);
    clone_skill(git_url, new_commit, skill_path_rel, &install_dir)?;

    let lifecycle_path = install_dir.join("lifecycle.yaml");
    if lifecycle_path.exists() {
        let yaml = std::fs::read_to_string(&lifecycle_path)?;
        let lf: LifecycleFile = lifecycle::parse(&yaml)?;
        let resolved_vars = vars::resolve(name, install_dir.to_str().unwrap_or(""), &lf.variables)?;
        execute_lifecycle(&lf.update, &resolved_vars, quiet, approver, sh_executor)?;
    }

    let mut lock = LockFile::load(lp)?;
    lock.insert(LockEntry {
        hub_id: existing.hub_id.clone(),
        slug: name.to_string(),
        version: new_version.to_string(),
        commit: new_commit.to_string(),
        installed_path: existing.installed_path.clone(),
        installed_at: Utc::now().to_rfc3339(),
    });
    lock.save(lp)?;

    println!("✓ Updated skill '{name}' to {new_version}");
    Ok(())
}

fn clone_skill(git_url: &str, commit: &str, skill_path_rel: &str, dest: &Path) -> Result<()> {
    let tmp_path = std::env::temp_dir().join(format!("agentctl-clone-{}", std::process::id()));
    std::fs::create_dir_all(&tmp_path)?;
    let repo = git2::Repository::clone(git_url, &tmp_path)?;
    if !commit.is_empty() {
        let obj = repo.revparse_single(commit)?;
        repo.checkout_tree(&obj, None)?;
    }
    let src = tmp_path.join(skill_path_rel);
    if !src.exists() {
        std::fs::remove_dir_all(&tmp_path)?;
        bail!("skill path '{skill_path_rel}' not found in repo");
    }
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    copy_dir(&src, dest)?;
    std::fs::remove_dir_all(&tmp_path)?;
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}
