use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEntry {
    pub hub_id: String,
    pub slug: String,
    pub version: String,
    pub commit: String,
    pub installed_path: String,
    pub installed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    pub version: String,
    pub skills: HashMap<String, LockEntry>,
}

impl Default for LockFile {
    fn default() -> Self {
        Self::new()
    }
}

impl LockFile {
    pub fn new() -> Self {
        Self {
            version: "1.0".into(),
            skills: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let s = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn insert(&mut self, entry: LockEntry) {
        let key = format!("{}:{}", entry.hub_id, entry.slug);
        self.skills.insert(key, entry);
    }

    pub fn remove(&mut self, hub_id: &str, slug: &str) -> bool {
        let key = format!("{hub_id}:{slug}");
        self.skills.remove(&key).is_some()
    }

    pub fn get(&self, hub_id: &str, slug: &str) -> Option<&LockEntry> {
        let key = format!("{hub_id}:{slug}");
        self.skills.get(&key)
    }
}

pub fn lock_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".agentctl")
        .join("skills.lock.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(hub: &str, slug: &str) -> LockEntry {
        LockEntry {
            hub_id: hub.into(),
            slug: slug.into(),
            version: "1.0.0".into(),
            commit: "abc1234".into(),
            installed_path: format!("skills/{slug}"),
            installed_at: "2026-07-15T00:00:00Z".into(),
        }
    }

    #[test]
    fn insert_and_get() {
        let mut lock = LockFile::new();
        lock.insert(entry("my-hub", "my-skill"));
        assert!(lock.get("my-hub", "my-skill").is_some());
    }

    #[test]
    fn remove_entry() {
        let mut lock = LockFile::new();
        lock.insert(entry("my-hub", "my-skill"));
        assert!(lock.remove("my-hub", "my-skill"));
        assert!(lock.get("my-hub", "my-skill").is_none());
    }

    #[test]
    fn roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("skills.lock.json");
        let mut lock = LockFile::new();
        lock.insert(entry("my-hub", "my-skill"));
        lock.save(&path).unwrap();
        let loaded = LockFile::load(&path).unwrap();
        assert!(loaded.get("my-hub", "my-skill").is_some());
    }

    #[test]
    fn load_missing_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let lock = LockFile::load(&path).unwrap();
        assert!(lock.skills.is_empty());
    }
}
