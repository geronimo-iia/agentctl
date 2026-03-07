use std::path::Path;

use anyhow::{bail, Result};

#[derive(Debug)]
pub struct ValidationError {
    pub file: String,
    pub line: Option<usize>,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{}: {}", self.file, line, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn validate_skills_hub(path: &Path) -> Result<ValidationResult> {
    let mut errors = Vec::new();

    let skill_dirs: Vec<_> = std::fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            if !p.is_dir() {
                return false;
            }
            // skip hidden directories (e.g. .git)
            e.file_name().to_string_lossy().starts_with('.') == false
        })
        .collect();

    if skill_dirs.is_empty() {
        bail!("no skill directories found in {}", path.display());
    }

    for entry in skill_dirs {
        let skill_dir = entry.path();
        let skill_md = skill_dir.join("SKILL.md");

        if !skill_md.exists() {
            errors.push(ValidationError {
                file: skill_dir.display().to_string(),
                line: None,
                message: "missing SKILL.md".into(),
            });
            continue;
        }

        // Flat hierarchy: no nested skill dirs
        for nested in std::fs::read_dir(&skill_dir)?.filter_map(|e| e.ok()) {
            if nested.path().is_dir() {
                let name = nested.file_name();
                let name = name.to_string_lossy();
                if !matches!(name.as_ref(), "scripts" | "references" | "assets") {
                    errors.push(ValidationError {
                        file: nested.path().display().to_string(),
                        line: None,
                        message: "nested skill directories not allowed (flat hierarchy rule)"
                            .into(),
                    });
                }
            }
        }

        errors.extend(validate_skill_frontmatter(&skill_md));
    }

    Ok(ValidationResult { errors })
}

pub fn validate_docs_hub(path: &Path) -> Result<ValidationResult> {
    let mut errors = Vec::new();

    let md_files: Vec<_> = glob_md_files(path);

    if md_files.is_empty() {
        bail!("no .md files found in {}", path.display());
    }

    for file in md_files {
        errors.extend(validate_doc_frontmatter(&file));
    }

    Ok(ValidationResult { errors })
}

fn validate_skill_frontmatter(path: &Path) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let file = path.display().to_string();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return vec![ValidationError {
                file,
                line: None,
                message: e.to_string(),
            }];
        }
    };

    let fm = match parse_frontmatter(&content) {
        Ok(fm) => fm,
        Err(e) => {
            return vec![ValidationError {
                file,
                line: Some(1),
                message: e,
            }];
        }
    };

    for field in ["name", "description"] {
        if !fm.contains_key(field)
            || fm[field].is_null()
            || fm[field].as_str().unwrap_or("").is_empty()
        {
            errors.push(ValidationError {
                file: file.clone(),
                line: Some(1),
                message: format!("missing required field: {field}"),
            });
        }
    }

    errors
}

fn validate_doc_frontmatter(path: &Path) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let file = path.display().to_string();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return vec![ValidationError {
                file,
                line: None,
                message: e.to_string(),
            }];
        }
    };

    if !content.starts_with("---") {
        errors.push(ValidationError {
            file,
            line: Some(1),
            message: "missing YAML frontmatter".into(),
        });
        return errors;
    }

    let fm = match parse_frontmatter(&content) {
        Ok(fm) => fm,
        Err(e) => {
            return vec![ValidationError {
                file,
                line: Some(1),
                message: e,
            }];
        }
    };

    for field in ["title", "summary", "status", "last_updated"] {
        if !fm.contains_key(field)
            || fm[field].is_null()
            || fm[field].as_str().unwrap_or("").is_empty()
        {
            errors.push(ValidationError {
                file: file.clone(),
                line: Some(1),
                message: format!("missing required field: {field}"),
            });
        }
    }

    if !fm.contains_key("read_when")
        || fm["read_when"]
            .as_sequence()
            .map(|s| s.is_empty())
            .unwrap_or(true)
    {
        errors.push(ValidationError {
            file: file.clone(),
            line: Some(1),
            message: "missing required field: read_when (must be non-empty list)".into(),
        });
    }

    errors
}

fn parse_frontmatter(content: &str) -> Result<serde_yaml::Mapping, String> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("invalid frontmatter: missing closing ---".into());
    }
    serde_yaml::from_str(parts[1]).map_err(|e| format!("invalid YAML: {e}"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn hidden_dirs_are_ignored() {
        let dir = tempdir().unwrap();
        // create a valid skill dir
        let skill_dir = dir.path().join("my-skill");
        fs::create_dir(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "---\nname: my-skill\ndescription: test\n---\n").unwrap();
        // create a hidden dir that should be ignored
        let git_dir = dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let result = validate_skills_hub(dir.path()).unwrap();
        assert!(result.is_valid(), "errors: {:?}", result.errors);
    }
}
