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
