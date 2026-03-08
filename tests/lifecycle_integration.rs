use agentctl::skill::lifecycle::{execute_lifecycle, execute_update, parse, sh_executor};
use agentctl::skill::vars;
use std::collections::HashMap;

fn skill_vars(skill_path: &str) -> HashMap<String, String> {
    vars::resolve("test-skill", skill_path, &HashMap::new()).unwrap()
}

/// install creates a marker file, update rewrites it, uninstall removes it
#[test]
fn lifecycle_install_update_uninstall() {
    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("marker.txt");

    let yaml = format!(
        r#"
install:
  - command: echo installed > {marker}
    description: Create marker
    requires_approval: false

update:
  - command: echo updated > {marker}
    description: Update marker
    requires_approval: false

uninstall:
  - command: rm {marker}
    description: Remove marker
    requires_approval: false
"#,
        marker = marker.display()
    );

    let lf = parse(&yaml).unwrap();
    let vars = skill_vars(dir.path().to_str().unwrap());

    // install
    execute_lifecycle(&lf.install, &vars, true, |_| true, sh_executor).unwrap();
    assert!(marker.exists(), "marker should exist after install");

    // update
    execute_update(&lf, &vars, true, false, |_| true, sh_executor).unwrap();
    let content = std::fs::read_to_string(&marker).unwrap();
    assert!(
        content.contains("updated"),
        "marker should contain 'updated' after update"
    );

    // uninstall
    execute_lifecycle(&lf.uninstall, &vars, true, |_| true, sh_executor).unwrap();
    assert!(!marker.exists(), "marker should be gone after uninstall");
}

/// --force on a skill with no update section runs uninstall then install
#[test]
fn lifecycle_force_reinstall() {
    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("marker.txt");

    let yaml = format!(
        r#"
install:
  - command: echo installed > {marker}
    description: Create marker
    requires_approval: false

uninstall:
  - command: rm -f {marker}
    description: Remove marker
    requires_approval: false
"#,
        marker = marker.display()
    );

    let lf = parse(&yaml).unwrap();
    let vars = skill_vars(dir.path().to_str().unwrap());

    // pre-create marker to simulate existing install
    std::fs::write(&marker, "old").unwrap();

    // force=true with no update section: uninstall removes, install recreates
    execute_update(&lf, &vars, true, true, |_| true, sh_executor).unwrap();
    assert!(
        marker.exists(),
        "marker should be recreated by force reinstall"
    );
    let content = std::fs::read_to_string(&marker).unwrap();
    assert!(content.contains("installed"));
}

/// --force=false with no update section errors
#[test]
fn lifecycle_no_update_section_errors() {
    let yaml = r#"
install:
  - command: echo hi
    description: Install
    requires_approval: false
"#;
    let lf = parse(yaml).unwrap();
    let vars = skill_vars("/tmp/test-skill");
    assert!(execute_update(&lf, &vars, true, false, |_| true, sh_executor).is_err());
}
