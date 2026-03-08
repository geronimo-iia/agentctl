mod common;

use tempfile::TempDir;

fn cfg(dir: &TempDir) -> std::process::Command {
    let mut cmd = common::agentctl();
    cmd.args(["--config", dir.path().join("config.json").to_str().unwrap()]);
    cmd
}

#[test]
fn path_prints_config_path() {
    let dir = TempDir::new().unwrap();
    let out = cfg(&dir).args(["config", "path"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.trim().ends_with("config.json"));
}

#[test]
fn init_creates_file() {
    let dir = TempDir::new().unwrap();
    let out = cfg(&dir).args(["config", "init"]).output().unwrap();
    assert!(out.status.success());
    assert!(dir.path().join("config.json").exists());
}

#[test]
fn init_errors_if_exists() {
    let dir = TempDir::new().unwrap();
    cfg(&dir).args(["config", "init"]).output().unwrap();
    let out = cfg(&dir).args(["config", "init"]).output().unwrap();
    assert!(!out.status.success());
}

#[test]
fn init_force_overwrites() {
    let dir = TempDir::new().unwrap();
    cfg(&dir).args(["config", "init"]).output().unwrap();
    let out = cfg(&dir)
        .args(["config", "init", "--force"])
        .output()
        .unwrap();
    assert!(out.status.success());
}

#[test]
fn show_prints_json() {
    let dir = TempDir::new().unwrap();
    cfg(&dir).args(["config", "init"]).output().unwrap();
    let out = cfg(&dir).args(["config", "show"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str::<serde_json::Value>(stdout.trim()).unwrap();
}

#[test]
fn get_unset_key_returns_empty() {
    let dir = TempDir::new().unwrap();
    let out = cfg(&dir)
        .args(["config", "get", "skills_root"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "");
}

#[test]
fn set_and_get_round_trip() {
    let dir = TempDir::new().unwrap();
    cfg(&dir)
        .args(["config", "set", "skills_root", "/tmp/skills"])
        .output()
        .unwrap();
    let out = cfg(&dir)
        .args(["config", "get", "skills_root"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "/tmp/skills");
}

#[test]
fn set_creates_file_if_missing() {
    let dir = TempDir::new().unwrap();
    assert!(!dir.path().join("config.json").exists());
    let out = cfg(&dir)
        .args(["config", "set", "skills_root", "/tmp/x"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(dir.path().join("config.json").exists());
}

#[test]
fn get_unknown_key_fails() {
    let dir = TempDir::new().unwrap();
    let out = cfg(&dir)
        .args(["config", "get", "nonexistent"])
        .output()
        .unwrap();
    assert!(!out.status.success());
}

#[test]
fn set_unknown_key_fails() {
    let dir = TempDir::new().unwrap();
    let out = cfg(&dir)
        .args(["config", "set", "nonexistent", "val"])
        .output()
        .unwrap();
    assert!(!out.status.success());
}
