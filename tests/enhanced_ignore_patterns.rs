use std::fs;
use tempfile::TempDir;

use agentctl::hub::config::HubConfig;

#[test]
fn directory_patterns_exclude_all_files_in_directory() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = ["templates/"]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Files in templates directory should be ignored
    assert!(cfg.is_ignored("templates/file.md"));
    assert!(cfg.is_ignored("templates/subdir/file.md"));

    // Files outside templates directory should not be ignored
    assert!(!cfg.is_ignored("docs/file.md"));
    assert!(!cfg.is_ignored("file.md"));
}

#[test]
fn path_wildcards_match_specific_patterns() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = ["rules/*.md"]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Files matching path pattern should be ignored
    assert!(cfg.is_ignored("rules/file.md"));
    assert!(cfg.is_ignored("rules/test.md"));

    // Files not matching pattern should not be ignored
    assert!(!cfg.is_ignored("rules/subdir/file.md")); // deeper nesting
    assert!(!cfg.is_ignored("docs/file.md")); // different directory
    assert!(!cfg.is_ignored("file.md")); // root level
}

#[test]
fn backward_compatibility_filename_patterns_still_work() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = ["draft-*.md"]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Filename patterns should work regardless of path
    assert!(cfg.is_ignored("draft-wip.md"));
    assert!(cfg.is_ignored("docs/draft-test.md"));
    assert!(cfg.is_ignored("a/b/c/draft-example.md"));

    // Non-matching filenames should not be ignored
    assert!(!cfg.is_ignored("final-doc.md"));
    assert!(!cfg.is_ignored("docs/published.md"));
}

#[test]
fn case_insensitive_matching_works() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = ["Rules/Templates/"]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Case variations should all match
    assert!(cfg.is_ignored("rules/templates/file.md"));
    assert!(cfg.is_ignored("Rules/Templates/file.md"));
    assert!(cfg.is_ignored("RULES/TEMPLATES/file.md"));
}

#[test]
fn path_separator_normalization_works() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = ["rules\\templates\\"]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Both forward and backslashes should work
    assert!(cfg.is_ignored("rules/templates/file.md"));
    assert!(cfg.is_ignored("rules\\templates\\file.md"));
}

#[test]
fn multiple_pattern_types_work_together() {
    let temp = TempDir::new().unwrap();
    let config_content = r#"
[generate]
ignore = [
    "README.md",
    "draft-*.md", 
    "templates/",
    "docs/private/"
]
"#;
    fs::write(temp.path().join("agentctl.toml"), config_content).unwrap();

    let cfg = HubConfig::load(temp.path());

    // Each pattern type should work
    assert!(cfg.is_ignored("README.md")); // exact filename
    assert!(cfg.is_ignored("draft-test.md")); // filename wildcard
    assert!(cfg.is_ignored("templates/file.md")); // directory
    assert!(cfg.is_ignored("docs/private/secret.md")); // directory

    // Non-matching files should not be ignored
    assert!(!cfg.is_ignored("docs/public/file.md"));
    assert!(!cfg.is_ignored("final-doc.md"));
}
