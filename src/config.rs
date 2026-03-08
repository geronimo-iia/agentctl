#![allow(dead_code)]

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
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_config(dir: &TempDir, json: &str) -> PathBuf {
        let path = dir.path().join("config.json");
        std::fs::write(&path, json).unwrap();
        path
    }

    #[test]
    fn roundtrip_empty() {
        let cfg = Config::default();
        let json = serde_json::to_string_pretty(&cfg).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert!(back.skill_hubs.is_empty());
        assert!(back.doc_hubs.is_empty());
    }

    #[test]
    fn defaults_applied_on_missing_fields() {
        let json = r#"{"skill_hubs":[{"id":"x","index_url":"http://x"}],"doc_hubs":[]}"#;
        let cfg: Config = serde_json::from_str(json).unwrap();
        assert!(cfg.skill_hubs[0].enabled);
        assert_eq!(cfg.skill_hubs[0].ttl_hours, 6);
        assert!(cfg.skill_hubs[0].git_url.is_none());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let entry = HubEntry {
            id: "agent-foundation".into(),
            index_url: "https://example.com/index.json".into(),
            git_url: Some("https://github.com/x/y".into()),
            enabled: true,
            ttl_hours: 12,
        };
        let cfg = Config {
            skill_hubs: vec![entry.clone()],
            doc_hubs: vec![],
        };
        let path = dir.path().join("config.json");
        std::fs::write(&path, serde_json::to_string_pretty(&cfg).unwrap()).unwrap();
        let loaded: Config =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.skill_hubs[0], entry);
    }
}
