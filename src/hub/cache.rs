#![allow(dead_code)]

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};

pub type Fetcher = fn(&str) -> Result<String>;

pub fn cache_dir_for(hub_id: &str) -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".agentctl")
        .join("cache")
        .join("hubs")
        .join(hub_id)
}

pub fn get(hub_id: &str, index_url: &str, ttl_hours: u64) -> Result<String> {
    get_from(
        &cache_dir_for(hub_id),
        index_url,
        ttl_hours,
        hub_id,
        http_fetch,
    )
}

pub fn refresh(hub_id: &str, index_url: &str) -> Result<String> {
    refresh_to(&cache_dir_for(hub_id), index_url, http_fetch)
}

pub fn get_from(
    dir: &Path,
    index_url: &str,
    ttl_hours: u64,
    hub_id: &str,
    fetcher: Fetcher,
) -> Result<String> {
    let index_path = dir.join("index.json");
    let fetched_at_path = dir.join("fetched_at");

    if !needs_refresh(&fetched_at_path, ttl_hours) {
        return Ok(std::fs::read_to_string(&index_path)?);
    }

    match fetcher(index_url) {
        Ok(body) => {
            std::fs::create_dir_all(dir)?;
            std::fs::write(&index_path, &body)?;
            std::fs::write(&fetched_at_path, Utc::now().to_rfc3339())?;
            Ok(body)
        }
        Err(e) => {
            if index_path.exists() {
                eprintln!("warning: fetch failed ({e}), using stale cache for '{hub_id}'");
                Ok(std::fs::read_to_string(&index_path)?)
            } else {
                Err(e.context(format!("fetch failed and no cache exists for '{hub_id}'")))
            }
        }
    }
}

pub fn refresh_to(dir: &Path, index_url: &str, fetcher: Fetcher) -> Result<String> {
    let body = fetcher(index_url)?;
    std::fs::create_dir_all(dir)?;
    std::fs::write(dir.join("index.json"), &body)?;
    std::fs::write(dir.join("fetched_at"), Utc::now().to_rfc3339())?;
    Ok(body)
}

fn needs_refresh(fetched_at_path: &Path, ttl_hours: u64) -> bool {
    let Ok(ts) = std::fs::read_to_string(fetched_at_path) else {
        return true;
    };
    let Ok(fetched_at) = ts.trim().parse::<DateTime<Utc>>() else {
        return true;
    };
    Utc::now() - fetched_at > Duration::hours(ttl_hours as i64)
}

pub fn http_fetch(url: &str) -> Result<String> {
    ureq::get(url)
        .call()
        .context("HTTP request failed")?
        .into_string()
        .context("failed to read response body")
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

    fn seed_cache(dir: &TempDir, fetched_at: &str) {
        let index = std::fs::read_to_string(fixture("cache-index.json")).unwrap();
        std::fs::write(dir.path().join("index.json"), index).unwrap();
        std::fs::write(dir.path().join("fetched_at"), fetched_at).unwrap();
    }

    fn ok_fetcher(_url: &str) -> Result<String> {
        Ok(std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cache-index.json"),
        )
        .unwrap())
    }

    fn err_fetcher(_url: &str) -> Result<String> {
        anyhow::bail!("HTTP request failed")
    }

    #[test]
    fn fresh_cache_does_not_need_refresh() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("fetched_at"), Utc::now().to_rfc3339()).unwrap();
        assert!(!needs_refresh(&dir.path().join("fetched_at"), 6));
    }

    #[test]
    fn missing_fetched_at_needs_refresh() {
        let dir = TempDir::new().unwrap();
        assert!(needs_refresh(&dir.path().join("fetched_at"), 6));
    }

    #[test]
    fn expired_cache_needs_refresh() {
        let dir = TempDir::new().unwrap();
        let old = (Utc::now() - Duration::hours(7)).to_rfc3339();
        std::fs::write(dir.path().join("fetched_at"), old).unwrap();
        assert!(needs_refresh(&dir.path().join("fetched_at"), 6));
    }

    #[test]
    fn corrupt_fetched_at_needs_refresh() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("fetched_at"), "not-a-date").unwrap();
        assert!(needs_refresh(&dir.path().join("fetched_at"), 6));
    }

    #[test]
    fn get_from_returns_fresh_cache_without_fetching() {
        let dir = TempDir::new().unwrap();
        seed_cache(&dir, &Utc::now().to_rfc3339());
        let result = get_from(dir.path(), "unused", 6, "test-hub", err_fetcher);
        assert!(result.unwrap().contains("test-hub"));
    }

    #[test]
    fn get_from_fetches_when_stale() {
        let dir = TempDir::new().unwrap();
        let old = (Utc::now() - Duration::hours(7)).to_rfc3339();
        seed_cache(&dir, &old);
        let result = get_from(dir.path(), "unused", 6, "test-hub", ok_fetcher);
        assert!(result.unwrap().contains("test-hub"));
    }

    #[test]
    fn get_from_uses_stale_cache_on_fetch_failure() {
        let dir = TempDir::new().unwrap();
        let old = (Utc::now() - Duration::hours(7)).to_rfc3339();
        seed_cache(&dir, &old);
        let result = get_from(dir.path(), "unused", 6, "test-hub", err_fetcher);
        assert!(result.unwrap().contains("test-hub"));
    }

    #[test]
    fn get_from_errors_when_no_cache_and_fetch_fails() {
        let dir = TempDir::new().unwrap();
        let result = get_from(dir.path(), "unused", 6, "test-hub", err_fetcher);
        assert!(result.is_err());
    }

    #[test]
    fn refresh_to_writes_cache_on_success() {
        let dir = TempDir::new().unwrap();
        refresh_to(dir.path(), "unused", ok_fetcher).unwrap();
        assert!(dir.path().join("index.json").exists());
        assert!(dir.path().join("fetched_at").exists());
    }

    #[test]
    fn refresh_to_errors_on_fetch_failure() {
        let dir = TempDir::new().unwrap();
        assert!(refresh_to(dir.path(), "unused", err_fetcher).is_err());
    }
}
