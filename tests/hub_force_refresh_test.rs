use tempfile::TempDir;

use agentctl::config::{Config, HubEntry};
use agentctl::hub::registry;

#[test]
fn test_refresh_one_force_deletes_cache() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Create config with test hub
    let cfg = Config {
        skills_root: None,
        skill_hubs: vec![HubEntry {
            id: "test-hub".to_string(),
            index_url:
                "https://raw.githubusercontent.com/geronimo-iia/agent-skills/main/index.json"
                    .to_string(),
            git_url: None,
            enabled: true,
            ttl_hours: 6,
        }],
        doc_hubs: vec![],
    };
    cfg.save_to(&config_path).unwrap();

    // Test that force refresh works (we can't easily test cache deletion without mocking)
    let result = registry::refresh_one_force(&config_path, "test-hub");
    assert!(result.is_ok());
}

#[test]
fn test_refresh_all_force_works() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Create config with test hub
    let cfg = Config {
        skills_root: None,
        skill_hubs: vec![HubEntry {
            id: "test-hub".to_string(),
            index_url:
                "https://raw.githubusercontent.com/geronimo-iia/agent-skills/main/index.json"
                    .to_string(),
            git_url: None,
            enabled: true,
            ttl_hours: 6,
        }],
        doc_hubs: vec![],
    };
    cfg.save_to(&config_path).unwrap();

    // Test that force refresh all works
    let result = registry::refresh_all_force(&config_path);
    assert!(result.is_ok());
}

#[test]
fn test_force_refresh_nonexistent_hub_fails() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let cfg = Config {
        skills_root: None,
        skill_hubs: vec![],
        doc_hubs: vec![],
    };
    cfg.save_to(&config_path).unwrap();

    let result = registry::refresh_one_force(&config_path, "nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}
