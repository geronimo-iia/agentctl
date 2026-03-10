use std::fs;
use tempfile::TempDir;

use agentctl::skill;

#[test]
fn test_skill_export() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("skills.lock.json");

    // Create a sample lock file
    let lock_content = r#"{
  "version": "1.0",
  "skills": {
    "test-hub:python-dev": {
      "hub_id": "test-hub",
      "slug": "python-dev",
      "version": "1.0.0",
      "commit": "abc123",
      "installed_path": "/home/user/.agent/skills/python-dev",
      "installed_at": "2025-01-16T12:00:00Z"
    }
  }
}"#;
    fs::write(&lock_path, lock_content).unwrap();

    // Test export functionality
    let result = skill::export(&lock_path);
    assert!(result.is_ok());
}

#[test]
fn test_skill_export_empty_lock() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("skills.lock.json");

    // Create empty lock file
    let lock_content = r#"{
  "version": "1.0",
  "skills": {}
}"#;
    fs::write(&lock_path, lock_content).unwrap();

    // Test export functionality with empty lock
    let result = skill::export(&lock_path);
    assert!(result.is_ok());
}

#[test]
fn test_skill_export_nonexistent_lock() {
    let temp_dir = TempDir::new().unwrap();
    let lock_path = temp_dir.path().join("nonexistent.lock.json");

    // Test export with nonexistent lock file (should create empty lock)
    let result = skill::export(&lock_path);
    assert!(result.is_ok());
}
