use anyhow::{bail, Context, Result};
use std::process::Command;

/// Get config option from git
pub fn get_config(name: &str) -> Result<String> {
    let full_name = format!("issue.{name}");

    let output = Command::new("git")
        .args(["config", &full_name])
        .output()
        .with_context(|| "cannot launch git config")?;

    if !output.status.success() {
        bail!("Cannot get property {full_name} from git");
    }

    let property = String::from_utf8(output.stdout)?;

    Ok(property)
}
