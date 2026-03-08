#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;

pub fn agentctl() -> Command {
    let bin = env!("CARGO_BIN_EXE_agentctl");
    Command::new(bin)
}

pub fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

pub fn with_config(dir: &tempfile::TempDir) -> Command {
    let mut cmd = agentctl();
    cmd.args(["--config", dir.path().join("config.json").to_str().unwrap()]);
    cmd
}

pub fn with_lock(dir: &tempfile::TempDir) -> Command {
    let mut cmd = agentctl();
    cmd.args([
        "--lock",
        dir.path().join("skills.lock.json").to_str().unwrap(),
    ]);
    cmd
}

pub fn with_config_and_lock(cfg_dir: &tempfile::TempDir, lock_dir: &tempfile::TempDir) -> Command {
    let mut cmd = agentctl();
    cmd.args([
        "--config",
        cfg_dir.path().join("config.json").to_str().unwrap(),
    ]);
    cmd.args([
        "--lock",
        lock_dir.path().join("skills.lock.json").to_str().unwrap(),
    ]);
    cmd
}
