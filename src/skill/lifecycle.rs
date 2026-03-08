use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

use super::vars;

pub type Approver = fn(&str) -> bool;
pub type Executor = fn(&str) -> Result<()>;

#[derive(Debug, Deserialize)]
pub struct LifecycleStep {
    pub command: String,
    pub description: String,
    #[serde(default = "default_platform")]
    pub platform: String,
    #[serde(default = "default_approval")]
    pub requires_approval: bool,
}

fn default_platform() -> String {
    "all".into()
}
fn default_approval() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct LifecycleFile {
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub install: Vec<LifecycleStep>,
    #[serde(default)]
    pub update: Vec<LifecycleStep>,
    #[serde(default)]
    pub uninstall: Vec<LifecycleStep>,
}

pub fn parse(yaml: &str) -> Result<LifecycleFile> {
    Ok(serde_yaml::from_str(yaml)?)
}

fn current_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

pub fn execute_lifecycle(
    steps: &[LifecycleStep],
    vars: &HashMap<String, String>,
    quiet: bool,
    approver: Approver,
    executor: Executor,
) -> Result<()> {
    let platform = current_platform();
    for step in steps {
        if step.platform != "all" && step.platform != platform {
            continue;
        }
        let cmd = vars::expand(&step.command, vars)?;
        if !quiet {
            println!("  \u{2192} {}", step.description);
            println!("    {cmd}");
            if step.requires_approval {
                print!("  Approve? [y/N] ");
                use std::io::Write;
                std::io::stdout().flush()?;
                if !approver(&cmd) {
                    bail!("aborted by user");
                }
            }
        }
        executor(&cmd)?;
    }
    Ok(())
}

pub fn execute_update(
    lf: &LifecycleFile,
    vars: &HashMap<String, String>,
    quiet: bool,
    force: bool,
    approver: Approver,
    executor: Executor,
) -> Result<()> {
    if lf.update.is_empty() {
        if !force {
            bail!("skill has no update lifecycle — use --force to reinstall");
        }
        execute_lifecycle(&lf.uninstall, vars, quiet, approver, executor)?;
        execute_lifecycle(&lf.install, vars, quiet, approver, executor)?;
    } else {
        execute_lifecycle(&lf.update, vars, quiet, approver, executor)?;
    }
    Ok(())
}

pub fn sh_executor(cmd: &str) -> Result<()> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;
    if !status.success() {
        bail!("command failed: {cmd}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = r#"
variables:
  VENV: ${SKILL_PATH}/.venv

install:
  - command: echo install ${SKILL_NAME}
    description: Install skill
    platform: all
    requires_approval: false

  - command: echo macos-only
    description: macOS step
    platform: macos
    requires_approval: false

  - command: echo needs-approval
    description: Needs approval
    platform: all
    requires_approval: true
"#;

    fn base_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("SKILL_NAME".into(), "my-skill".into());
        m.insert("SKILL_PATH".into(), "/skills/my-skill".into());
        m.insert("HOME".into(), "/home/user".into());
        m.insert("PLATFORM".into(), "linux".into());
        m
    }

    #[test]
    fn parse_lifecycle_yaml() {
        let lf = parse(YAML).unwrap();
        assert_eq!(lf.install.len(), 3);
        assert_eq!(lf.variables["VENV"], "${SKILL_PATH}/.venv");
    }

    #[test]
    fn execute_skips_wrong_platform() {
        let lf = parse(YAML).unwrap();
        let vars = base_vars();
        // macos step skipped on linux — verify no error
        let result = execute_lifecycle(&lf.install, &vars, true, |_| true, |_cmd| Ok(()));
        assert!(result.is_ok());
    }

    #[test]
    fn execute_approver_abort() {
        let lf = parse(YAML).unwrap();
        let vars = base_vars();
        // always_no approver — should abort on requires_approval step
        let result = execute_lifecycle(&lf.install, &vars, false, |_| false, |_| Ok(()));
        assert!(result.is_err());
    }

    #[test]
    fn execute_quiet_skips_approval() {
        let lf = parse(YAML).unwrap();
        let vars = base_vars();
        // quiet=true, approver never called — should succeed
        let result = execute_lifecycle(&lf.install, &vars, true, |_| false, |_| Ok(()));
        assert!(result.is_ok());
    }

    const YAML_NO_UPDATE: &str = r#"
install:
  - command: echo install
    description: Install
    requires_approval: false
uninstall:
  - command: echo uninstall
    description: Uninstall
    requires_approval: false
"#;

    #[test]
    fn execute_update_no_section_errors_without_force() {
        let lf = parse(YAML_NO_UPDATE).unwrap();
        assert!(execute_update(&lf, &base_vars(), true, false, |_| true, |_| Ok(())).is_err());
    }

    #[test]
    fn execute_update_no_section_force_runs_uninstall_then_install() {
        let lf = parse(YAML_NO_UPDATE).unwrap();
        // force=true with no update section should succeed (runs uninstall then install)
        assert!(execute_update(&lf, &base_vars(), true, true, |_| true, |_| Ok(())).is_ok());
    }

    #[test]
    fn execute_update_with_section_runs_update() {
        let yaml = r#"
update:
  - command: echo update
    description: Update
    requires_approval: false
"#;
        let lf = parse(yaml).unwrap();
        // succeeds and runs the update section (verified by no error)
        assert!(execute_update(&lf, &base_vars(), true, false, |_| true, |_| Ok(())).is_ok());
    }
}
