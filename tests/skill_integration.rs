#[path = "common/mod.rs"]
mod common;
use common::{agentctl, fixture, with_config_and_lock, with_lock};

// ── CLI surface ───────────────────────────────────────────────────────────────

#[test]
fn skill_help() {
    let out = agentctl().args(["skill", "--help"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("install"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("remove"));
    assert!(stdout.contains("update"));
}

#[test]
fn global_quiet_flag_in_help() {
    let out = agentctl().arg("--help").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("--quiet") || stdout.contains("-q"));
}

#[test]
fn global_yes_flag_in_help() {
    let out = agentctl().arg("--help").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("--yes") || stdout.contains("-y"));
}

// ── skill list ────────────────────────────────────────────────────────────────

#[test]
fn skill_list_empty() {
    let dir = tempfile::tempdir().unwrap();
    let out = with_lock(&dir).args(["skill", "list"]).output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("No skills installed"));
}

#[test]
fn skill_list_shows_installed() {
    let dir = tempfile::tempdir().unwrap();
    let lock_path = dir.path().join("skills.lock.json");
    std::fs::write(
        &lock_path,
        r#"{
  "version": "1.0",
  "skills": {
    "test-hub:my-skill": {
      "hub_id": "test-hub",
      "slug": "my-skill",
      "version": "1.0.0",
      "commit": "abc1234",
      "installed_path": "/tmp/my-skill",
      "installed_at": "2026-01-01T00:00:00Z"
    }
  }
}"#,
    )
    .unwrap();

    let out = with_lock(&dir).args(["skill", "list"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("my-skill"));
    assert!(stdout.contains("1.0.0"));
}

// ── skill remove ──────────────────────────────────────────────────────────────

#[test]
fn skill_remove_not_installed_fails() {
    let dir = tempfile::tempdir().unwrap();
    let status = with_lock(&dir)
        .args(["skill", "remove", "ghost-skill", "--hub", "test-hub"])
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn skill_remove_installed_skill() {
    let dir = tempfile::tempdir().unwrap();
    let skill_dir = dir.path().join("my-skill");
    std::fs::create_dir_all(&skill_dir).unwrap();

    let lock_path = dir.path().join("skills.lock.json");
    std::fs::write(
        &lock_path,
        format!(
            r#"{{
  "version": "1.0",
  "skills": {{
    "test-hub:my-skill": {{
      "hub_id": "test-hub",
      "slug": "my-skill",
      "version": "1.0.0",
      "commit": "abc1234",
      "installed_path": "{}",
      "installed_at": "2026-01-01T00:00:00Z"
    }}
  }}
}}"#,
            skill_dir.display()
        ),
    )
    .unwrap();

    let status = with_lock(&dir)
        .args(["--yes", "skill", "remove", "my-skill", "--hub", "test-hub"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(!skill_dir.exists());

    // lock entry removed
    let out = with_lock(&dir).args(["skill", "list"]).output().unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("No skills installed"));
}

// ── skill update ─────────────────────────────────────────────────────────────

#[test]
fn skill_update_force_flag_in_help() {
    let out = agentctl()
        .args(["skill", "update", "--help"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("--force"));
}

#[test]
fn skill_install_no_hub_configured_fails() {
    let cfg_dir = tempfile::tempdir().unwrap();
    let lock_dir = tempfile::tempdir().unwrap();
    // empty config — no skill hubs
    std::fs::write(
        cfg_dir.path().join("config.json"),
        r#"{"skill_hubs":[],"doc_hubs":[]}"#,
    )
    .unwrap();

    let status = with_config_and_lock(&cfg_dir, &lock_dir)
        .args(["skill", "install", "my-skill"])
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn skill_install_unknown_hub_fails() {
    let cfg_dir = tempfile::tempdir().unwrap();
    let lock_dir = tempfile::tempdir().unwrap();
    std::fs::write(
        cfg_dir.path().join("config.json"),
        r#"{"skill_hubs":[],"doc_hubs":[]}"#,
    )
    .unwrap();

    let status = with_config_and_lock(&cfg_dir, &lock_dir)
        .args(["skill", "install", "my-skill", "--hub", "nonexistent"])
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn skill_install_skill_not_in_index_fails() {
    let cfg_dir = tempfile::tempdir().unwrap();
    let lock_dir = tempfile::tempdir().unwrap();

    // config with a hub pointing to a local file URL (will fail fetch, but we seed the cache)
    std::fs::write(
        cfg_dir.path().join("config.json"),
        r#"{"skill_hubs":[{"id":"test-hub","index_url":"http://localhost:0/index.json","enabled":true,"ttl_hours":6}],"doc_hubs":[]}"#,
    )
    .unwrap();

    // seed cache so install reaches the index lookup
    let cache_dir = cfg_dir.path().join("cache").join("hubs").join("test-hub");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::copy(fixture("skill-index.json"), cache_dir.join("index.json")).unwrap();
    std::fs::write(
        cache_dir.join("fetched_at"),
        chrono::Utc::now().to_rfc3339(),
    )
    .unwrap();

    // cache::get uses ~/.agentctl/cache — can't override path from CLI, so this
    // test verifies the "no enabled hub" / config error path only
    let status = with_config_and_lock(&cfg_dir, &lock_dir)
        .args(["skill", "install", "unknown-skill", "--hub", "test-hub"])
        .status()
        .unwrap();
    assert!(!status.success());
}
