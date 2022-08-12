use std::process::Command;

use anyhow::{bail, Context, Result};

/// Check that we are in a git repository
pub fn check_is_git() -> Result<bool> {
    let r = Command::new("git")
        .args(["status"])
        .output()
        .with_context(|| "cannot launch git status")?
        .status
        .success();
    Ok(r)
}

/// Get config option from git
pub fn get_config(name: &str) -> Result<String> {
    get_config_internal(name, None)
}

/// Get config option from git, with scope)
pub fn get_config_scoped(name: &str, scope: &str) -> Result<String> {
    get_config_internal(name, Some(scope))
}

/// Get config option from git (implementation)
fn get_config_internal(name: &str, scope: Option<&str>) -> Result<String> {
    let full_name = if let Some(scope) = scope {
        format!("issue.{scope}.{name}")
    } else {
        format!("issue.{name}")
    };

    let output = Command::new("git")
        .args(["config", &full_name])
        .output()
        .with_context(|| "cannot launch git config")?;

    if !output.status.success() {
        bail!("cannot get property {full_name} from git");
    }

    let property = String::from_utf8(output.stdout)?.trim().to_string();

    Ok(property)
}

/// switch to new branch
pub fn create_branch(branch: &str) -> Result<()> {
    let branch = branch.replace(" ", "_");
    let output = Command::new("git")
        .args(["checkout", "-b", &branch])
        .output()
        .with_context(|| "cannot launch git checkout")?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}
