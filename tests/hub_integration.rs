use std::path::PathBuf;
use std::process::Command;

fn agentctl() -> Command {
    let bin = env!("CARGO_BIN_EXE_agentctl");
    Command::new(bin)
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// ── CLI flags ─────────────────────────────────────────────────────────────────

#[test]
fn version_flag() {
    let output = agentctl().arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("0.1.0"));
}

#[test]
fn help_flag() {
    let output = agentctl().arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("hub"));
}

#[test]
fn hub_help_flag() {
    let output = agentctl().args(["hub", "--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("generate"));
}

// ── Skills hub ────────────────────────────────────────────────────────────────

#[test]
fn validate_skills_hub_valid() {
    // only my-skill passes; bad-skill has wrong fields — use a single-skill fixture
    let status = agentctl()
        .args(["hub", "validate", "--type", "skills", "--path"])
        .arg(fixture("skills-hub-valid"))
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn validate_skills_hub_rejects_bad_frontmatter() {
    let status = agentctl()
        .args(["hub", "validate", "--type", "skills", "--path"])
        .arg(fixture("skills-hub-invalid"))
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn validate_skills_hub_ignores_git_dir() {
    // skills-hub contains .git/ — validation must still pass on valid skills
    let status = agentctl()
        .args(["hub", "validate", "--type", "skills", "--path"])
        .arg(fixture("skills-hub-valid"))
        .status()
        .unwrap();
    assert!(status.success());
}

// ── Docs hub ──────────────────────────────────────────────────────────────────

#[test]
fn validate_docs_hub_valid() {
    let status = agentctl()
        .args(["hub", "validate", "--type", "docs", "--path"])
        .arg(fixture("docs-hub"))
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn validate_docs_hub_rejects_missing_fields() {
    let status = agentctl()
        .args(["hub", "validate", "--type", "docs", "--path"])
        .arg(fixture("docs-hub-invalid"))
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn generate_docs_index() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("index.json");

    let status = agentctl()
        .args(["hub", "generate", "--type", "docs", "--path"])
        .arg(fixture("docs-hub"))
        .arg("--output")
        .arg(&output)
        .status()
        .unwrap();

    assert!(status.success());
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&output).unwrap()).unwrap();
    assert_eq!(json["type"], "docs");
    assert!(json["metadata"]["total_entries"].as_u64().unwrap() > 0);
}

// ── Hub registry ──────────────────────────────────────────────────────────────

fn with_config(dir: &tempfile::TempDir) -> std::process::Command {
    let mut cmd = agentctl();
    cmd.args(["--config", dir.path().join("config.json").to_str().unwrap()]);
    cmd
}

#[test]
fn hub_add_and_list() {
    let dir = tempfile::tempdir().unwrap();
    let status = with_config(&dir)
        .args([
            "hub",
            "add",
            "--type",
            "skills",
            "my-hub",
            "https://example.com/index.json",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let output = with_config(&dir).args(["hub", "list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("my-hub"));
}

#[test]
fn hub_add_duplicate_fails() {
    let dir = tempfile::tempdir().unwrap();
    with_config(&dir)
        .args([
            "hub",
            "add",
            "--type",
            "skills",
            "my-hub",
            "https://example.com/index.json",
        ])
        .status()
        .unwrap();
    let status = with_config(&dir)
        .args([
            "hub",
            "add",
            "--type",
            "skills",
            "my-hub",
            "https://example.com/index.json",
        ])
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn hub_remove() {
    let dir = tempfile::tempdir().unwrap();
    with_config(&dir)
        .args([
            "hub",
            "add",
            "--type",
            "skills",
            "my-hub",
            "https://example.com/index.json",
        ])
        .status()
        .unwrap();
    let status = with_config(&dir)
        .args(["hub", "remove", "--type", "skills", "my-hub"])
        .status()
        .unwrap();
    assert!(status.success());

    let output = with_config(&dir).args(["hub", "list"]).output().unwrap();
    assert!(!String::from_utf8_lossy(&output.stdout).contains("my-hub"));
}

#[test]
fn hub_enable_disable() {
    let dir = tempfile::tempdir().unwrap();
    with_config(&dir)
        .args([
            "hub",
            "add",
            "--type",
            "skills",
            "my-hub",
            "https://example.com/index.json",
        ])
        .status()
        .unwrap();

    let status = with_config(&dir)
        .args(["hub", "disable", "--type", "skills", "my-hub"])
        .status()
        .unwrap();
    assert!(status.success());
    let out = with_config(&dir).args(["hub", "list"]).output().unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("disabled"));

    let status = with_config(&dir)
        .args(["hub", "enable", "--type", "skills", "my-hub"])
        .status()
        .unwrap();
    assert!(status.success());
    let out = with_config(&dir).args(["hub", "list"]).output().unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("enabled"));
}
