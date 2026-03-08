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

    /// Returns true if the filename should be excluded (case-insensitive).
    pub fn is_ignored(&self, filename: &str) -> bool {
        let lower = filename.to_lowercase();
        self.ignore_list()
            .iter()
            .any(|pattern| glob_match(pattern, &lower))
    }
}

/// Minimal glob: supports `*` wildcard on filename only.
fn glob_match(pattern: &str, name: &str) -> bool {
    let pattern = pattern.to_lowercase();
    match pattern.find('*') {
        None => name == pattern.as_str(),
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
