use anyhow::{bail, Result};
use std::collections::HashMap;

pub fn expand(template: &str, vars: &HashMap<String, String>) -> Result<String> {
    let mut result = template.to_string();
    // iterate to handle chained substitutions in a single pass
    let mut i = 0;
    let bytes = result.as_bytes().to_vec();
    let mut out = String::with_capacity(result.len());
    let s = std::str::from_utf8(&bytes).unwrap();
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1] == '{' {
            let start = i + 2;
            if let Some(end) = chars[start..].iter().position(|&c| c == '}') {
                let name: String = chars[start..start + end].iter().collect();
                match vars.get(&name) {
                    Some(val) => out.push_str(val),
                    None => bail!("undefined variable: ${{{}}}", name),
                }
                i = start + end + 1;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    result = out;
    Ok(result)
}

pub fn resolve(
    skill_name: &str,
    skill_path: &str,
    custom: &HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    vars.insert("SKILL_NAME".into(), skill_name.into());
    vars.insert("SKILL_PATH".into(), skill_path.into());
    vars.insert("HOME".into(), std::env::var("HOME").unwrap_or_default());
    vars.insert(
        "PLATFORM".into(),
        if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        }
        .into(),
    );
    // custom vars evaluated in declaration order (HashMap doesn't preserve order,
    // caller must pass an IndexMap or pre-ordered vec — here we accept HashMap for simplicity)
    for (k, v) in custom {
        let expanded = expand(v, &vars)?;
        vars.insert(k.clone(), expanded);
    }
    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("SKILL_PATH".into(), "/skills/my-skill".into());
        m.insert("HOME".into(), "/home/user".into());
        m
    }

    #[test]
    fn expand_builtin() {
        let vars = base_vars();
        assert_eq!(
            expand("${SKILL_PATH}/.venv", &vars).unwrap(),
            "/skills/my-skill/.venv"
        );
    }

    #[test]
    fn expand_multiple() {
        let vars = base_vars();
        assert_eq!(
            expand("${HOME}/${SKILL_PATH}", &vars).unwrap(),
            "/home/user//skills/my-skill"
        );
    }

    #[test]
    fn expand_undefined_errors() {
        let vars = base_vars();
        assert!(expand("${UNDEFINED}", &vars).is_err());
    }

    #[test]
    fn resolve_builtins_present() {
        let vars = resolve("my-skill", "/skills/my-skill", &HashMap::new()).unwrap();
        assert_eq!(vars["SKILL_NAME"], "my-skill");
        assert_eq!(vars["SKILL_PATH"], "/skills/my-skill");
        assert!(vars.contains_key("HOME"));
        assert!(vars.contains_key("PLATFORM"));
    }

    #[test]
    fn resolve_custom_expands_against_builtins() {
        let mut custom = HashMap::new();
        custom.insert("VENV".into(), "${SKILL_PATH}/.venv".into());
        let vars = resolve("s", "/p", &custom).unwrap();
        assert_eq!(vars["VENV"], "/p/.venv");
    }

    #[test]
    fn resolve_custom_undefined_ref_errors() {
        let mut custom = HashMap::new();
        custom.insert("X".into(), "${UNDEFINED}".into());
        assert!(resolve("s", "/p", &custom).is_err());
    }
}
