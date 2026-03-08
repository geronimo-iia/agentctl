use std::path::Path;

use serde::Deserialize;

const DEFAULT_IGNORE: &[&str] = &[
    "README.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "ARCHIVED.md",
];

#[derive(Debug, Default, Deserialize)]
pub struct HubConfig {
    #[serde(default)]
    pub hub: HubSection,
    #[serde(default)]
    pub generate: GenerateSection,
}

#[derive(Debug, Default, Deserialize)]
pub struct HubSection {
    pub id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct GenerateSection {
    pub ignore: Option<Vec<String>>,
}

impl HubConfig {
    /// Load from `agentctl.toml` at hub root, or return defaults if absent.
    pub fn load(hub_path: &Path) -> Self {
        let toml_path = hub_path.join("agentctl.toml");
        if !toml_path.exists() {
            return Self::default();
        }
        let content = match std::fs::read_to_string(&toml_path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }

    /// Effective ignore list: from config if set, otherwise defaults.
    pub fn ignore_list(&self) -> Vec<String> {
        match &self.generate.ignore {
            Some(list) => list.clone(),
            None => DEFAULT_IGNORE.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Returns true if the file path should be excluded (case-insensitive).
    pub fn is_ignored(&self, file_path: &str) -> bool {
        let lower = file_path.to_lowercase().replace('\\', "/"); // normalize path separators
        self.ignore_list()
            .iter()
            .any(|pattern| glob_match(pattern, &lower))
    }
}

/// Enhanced glob matching: supports path patterns, directories, and wildcards.
fn glob_match(pattern: &str, path: &str) -> bool {
    let pattern = pattern.to_lowercase().replace('\\', "/"); // normalize separators

    // Handle directory patterns (ending with /)
    if pattern.ends_with('/') {
        let dir_name = pattern.trim_end_matches('/');

        // Check if path starts with this directory
        if path.starts_with(&format!("{}/", dir_name)) {
            return true;
        }

        // Check if path contains this directory
        if path.contains(&format!("/{}/", dir_name)) {
            return true;
        }

        return false;
    }

    // Handle simple path patterns with *
    if pattern.contains('/') {
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return false;
        }

        for (pattern_part, path_part) in pattern_parts.iter().zip(path_parts.iter()) {
            if !simple_glob_match(pattern_part, path_part) {
                return false;
            }
        }

        return true;
    }

    // Backward compatibility: filename-only patterns
    let filename = path.split('/').next_back().unwrap_or(path);
    simple_glob_match(&pattern, filename)
}

/// Simple glob matching for single component (filename or directory name)
fn simple_glob_match(pattern: &str, name: &str) -> bool {
    match pattern.find('*') {
        None => name == pattern,
        Some(i) => name.starts_with(&pattern[..i]) && name.ends_with(&pattern[i + 1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ignore_list() {
        let cfg = HubConfig::default();
        assert!(cfg.is_ignored("README.md"));
        assert!(cfg.is_ignored("readme.md")); // case-insensitive
        assert!(cfg.is_ignored("CHANGELOG.md"));
        assert!(!cfg.is_ignored("my-doc.md"));
    }

    #[test]
    fn custom_ignore_overrides_defaults() {
        let cfg = HubConfig {
            hub: HubSection { id: None },
            generate: GenerateSection {
                ignore: Some(vec!["draft-*.md".to_string()]),
            },
        };
        assert!(!cfg.is_ignored("README.md")); // not in custom list
        assert!(cfg.is_ignored("draft-wip.md"));
    }

    #[test]
    fn license_glob() {
        let cfg = HubConfig::default();
        // LICENSE* not in default list — only exact matches by default
        assert!(!cfg.is_ignored("LICENSE-MIT"));
    }
}
