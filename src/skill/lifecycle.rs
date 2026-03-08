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
}
