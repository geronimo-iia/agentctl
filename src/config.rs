use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HubEntry {
    pub id: String,
    pub index_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_ttl")]
    pub ttl_hours: u64,
}

fn default_true() -> bool {
    true
}

fn default_ttl() -> u64 {
    6
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub skill_hubs: Vec<HubEntry>,
    #[serde(default)]
    pub doc_hubs: Vec<HubEntry>,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".agentctl")
        .join("config.json")
}

impl Config {
    #[allow(dead_code)]
    pub fn load() -> Result<Self> {
        Self::load_from(&config_path())
    }

    pub fn load_from(path: &std::path::Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        self.save_to(&config_path())
    }

    pub fn save_to(&self, path: &std::path::Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn load_from_missing_returns_default() {
        let dir = TempDir::new().unwrap();
        let cfg = Config::load_from(&dir.path().join("config.json")).unwrap();
        assert!(cfg.skill_hubs.is_empty());
        assert!(cfg.doc_hubs.is_empty());
    }

    #[test]
    fn empty_json_object_loads_as_empty_config() {
        let cfg = Config::load_from(&fixture("config-empty.json")).unwrap();
        assert!(cfg.skill_hubs.is_empty());
        assert!(cfg.doc_hubs.is_empty());
    }

    #[test]
    fn defaults_applied_on_missing_fields() {
        let cfg = Config::load_from(&fixture("config-defaults.json")).unwrap();
        assert_eq!(cfg.skill_hubs[0].id, "minimal-hub");
        assert!(cfg.skill_hubs[0].enabled);
        assert_eq!(cfg.skill_hubs[0].ttl_hours, 6);
        assert!(cfg.skill_hubs[0].git_url.is_none());
    }

    #[test]
    fn load_valid_config() {
        let cfg = Config::load_from(&fixture("config-valid.json")).unwrap();
        assert_eq!(cfg.skill_hubs.len(), 1);
        assert_eq!(cfg.skill_hubs[0].id, "agent-foundation");
        assert_eq!(cfg.skill_hubs[0].ttl_hours, 12);
        assert!(cfg.skill_hubs[0].git_url.is_some());
        assert_eq!(cfg.doc_hubs.len(), 1);
        assert!(!cfg.doc_hubs[0].enabled);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let src = Config::load_from(&fixture("config-valid.json")).unwrap();
        src.save_to(&path).unwrap();
        let loaded = Config::load_from(&path).unwrap();
        assert_eq!(loaded.skill_hubs[0], src.skill_hubs[0]);
        assert_eq!(loaded.doc_hubs[0], src.doc_hubs[0]);
    }
}
