#![allow(dead_code)]

use std::path::Path;

use anyhow::{bail, Result};

use crate::config::{Config, HubEntry};
use crate::hub::cache::{self, Fetcher};

pub enum HubKind {
    Skill,
    Doc,
}

fn hubs_mut<'a>(cfg: &'a mut Config, kind: &HubKind) -> &'a mut Vec<HubEntry> {
    match kind {
        HubKind::Skill => &mut cfg.skill_hubs,
        HubKind::Doc => &mut cfg.doc_hubs,
    }
}

pub fn add(
    config_path: &Path,
    kind: HubKind,
    id: &str,
    index_url: &str,
    git_url: Option<&str>,
) -> Result<()> {
    let mut cfg = Config::load_from(config_path)?;
    let hubs = hubs_mut(&mut cfg, &kind);
    if hubs.iter().any(|h| h.id == id) {
        bail!("hub '{id}' already exists");
    }
    hubs.push(HubEntry {
        id: id.to_string(),
        index_url: index_url.to_string(),
        git_url: git_url.map(str::to_string),
        enabled: true,
        ttl_hours: 6,
    });
    cfg.save_to(config_path)
}

pub fn remove(config_path: &Path, kind: HubKind, id: &str) -> Result<()> {
    let mut cfg = Config::load_from(config_path)?;
    let hubs = hubs_mut(&mut cfg, &kind);
    let before = hubs.len();
    hubs.retain(|h| h.id != id);
    if hubs.len() == before {
        bail!("hub '{id}' not found");
    }
    cfg.save_to(config_path)
}

pub fn set_enabled(config_path: &Path, kind: HubKind, id: &str, enabled: bool) -> Result<()> {
    let mut cfg = Config::load_from(config_path)?;
    let hub = hubs_mut(&mut cfg, &kind)
        .iter_mut()
        .find(|h| h.id == id)
        .ok_or_else(|| anyhow::anyhow!("hub '{id}' not found"))?;
    hub.enabled = enabled;
    cfg.save_to(config_path)
}

pub fn refresh_one(config_path: &Path, id: &str) -> Result<()> {
    refresh_one_with(config_path, id, None, cache::http_fetch)
}

pub fn refresh_one_with(
    config_path: &Path,
    id: &str,
    cache_root: Option<&Path>,
    fetcher: Fetcher,
) -> Result<()> {
    let cfg = Config::load_from(config_path)?;
    let hub = cfg
        .skill_hubs
        .iter()
        .chain(cfg.doc_hubs.iter())
        .find(|h| h.id == id)
        .ok_or_else(|| anyhow::anyhow!("hub '{id}' not found"))?;
    let dir = cache_root
        .map(|r| r.join(&hub.id))
        .unwrap_or_else(|| cache::cache_dir_for(&hub.id));
    cache::refresh_to(&dir, &hub.index_url, fetcher)?;
    Ok(())
}

pub fn refresh_all(config_path: &Path) -> Result<()> {
    refresh_all_with(config_path, None, cache::http_fetch)
}

pub fn refresh_all_with(
    config_path: &Path,
    cache_root: Option<&Path>,
    fetcher: Fetcher,
) -> Result<()> {
    let cfg = Config::load_from(config_path)?;
    for hub in cfg
        .skill_hubs
        .iter()
        .chain(cfg.doc_hubs.iter())
        .filter(|h| h.enabled)
    {
        let dir = cache_root
            .map(|r| r.join(&hub.id))
            .unwrap_or_else(|| cache::cache_dir_for(&hub.id));
        if let Err(e) = cache::refresh_to(&dir, &hub.index_url, fetcher) {
            eprintln!("warning: failed to refresh '{}': {e}", hub.id);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;

    fn fixture(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    fn seed(dir: &TempDir) -> std::path::PathBuf {
        let src = std::fs::read_to_string(fixture("config-valid.json")).unwrap();
        let path = dir.path().join("config.json");
        std::fs::write(&path, src).unwrap();
        path
    }

    fn ok_fetcher(_url: &str) -> Result<String> {
        Ok(std::fs::read_to_string(fixture("cache-index.json")).unwrap())
    }

    fn err_fetcher(_url: &str) -> Result<String> {
        anyhow::bail!("mock fetch error")
    }

    fn hub_entry(id: &str, enabled: bool) -> HubEntry {
        HubEntry {
            id: id.to_string(),
            index_url: "https://unused.example.com/index.json".to_string(),
            git_url: None,
            enabled,
            ttl_hours: 6,
        }
    }

    #[test]
    fn add_new_hub() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        add(
            &path,
            HubKind::Skill,
            "new-hub",
            "https://example.com/index.json",
            None,
        )
        .unwrap();
        let cfg = Config::load_from(&path).unwrap();
        assert_eq!(cfg.skill_hubs.len(), 2);
        assert_eq!(cfg.skill_hubs[1].id, "new-hub");
    }

    #[test]
    fn add_duplicate_errors() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        let err = add(
            &path,
            HubKind::Skill,
            "agent-foundation",
            "https://x.com",
            None,
        )
        .unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn remove_existing_hub() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        remove(&path, HubKind::Skill, "agent-foundation").unwrap();
        let cfg = Config::load_from(&path).unwrap();
        assert!(cfg.skill_hubs.is_empty());
    }

    #[test]
    fn remove_missing_errors() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        let err = remove(&path, HubKind::Skill, "no-such-hub").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn disable_and_enable_hub() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        set_enabled(&path, HubKind::Skill, "agent-foundation", false).unwrap();
        assert!(!Config::load_from(&path).unwrap().skill_hubs[0].enabled);
        set_enabled(&path, HubKind::Skill, "agent-foundation", true).unwrap();
        assert!(Config::load_from(&path).unwrap().skill_hubs[0].enabled);
    }

    #[test]
    fn refresh_one_missing_hub_errors() {
        let dir = TempDir::new().unwrap();
        let path = seed(&dir);
        let err = refresh_one_with(&path, "no-such-hub", Some(dir.path()), ok_fetcher).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn refresh_one_fetch_error_propagates() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.json");
        let cfg = Config {
            skill_hubs: vec![hub_entry("test-hub", true)],
            doc_hubs: vec![],
        };
        cfg.save_to(&config_path).unwrap();
        let err =
            refresh_one_with(&config_path, "test-hub", Some(dir.path()), err_fetcher).unwrap_err();
        assert!(err.to_string().contains("mock fetch error"));
    }

    #[test]
    fn refresh_one_writes_cache_on_success() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.json");
        let cache_root = dir.path().join("cache");
        let cfg = Config {
            skill_hubs: vec![hub_entry("test-hub", true)],
            doc_hubs: vec![],
        };
        cfg.save_to(&config_path).unwrap();
        refresh_one_with(&config_path, "test-hub", Some(&cache_root), ok_fetcher).unwrap();
        assert!(cache_root.join("test-hub").join("index.json").exists());
    }

    #[test]
    fn refresh_all_skips_disabled_hubs() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.json");
        let cache_root = dir.path().join("cache");
        let cfg = Config {
            skill_hubs: vec![
                hub_entry("enabled-hub", true),
                hub_entry("disabled-hub", false),
            ],
            doc_hubs: vec![],
        };
        cfg.save_to(&config_path).unwrap();
        refresh_all_with(&config_path, Some(&cache_root), ok_fetcher).unwrap();
        assert!(cache_root.join("enabled-hub").join("index.json").exists());
        assert!(!cache_root.join("disabled-hub").exists());
    }
}
