const TEMPLATE_FILE: &str = ".git/issue.template";

use std::{
    fs::{OpenOptions, remove_file},
    io::Write,
    process::{Command, Output},
};

use anyhow::{bail, Context, Result};

/// Check that we are in a git repository
pub fn check_is_git() -> Result<bool> {
    let r = git(&["status"])?.status.success();
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

    let output = git(&["config", &full_name])?;

    if !output.status.success() {
        bail!("cannot get property {full_name} from git");
    }

    let property = String::from_utf8(output.stdout)?.trim().to_string();

    Ok(property)
}

/// Switch to new branch
pub fn create_branch(branch: &str) -> Result<()> {
    let output = git(&["checkout", "-b", branch])?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

/// Go back to master
pub fn clear_branch() -> Result<()> {
    let output = git(&["checkout", "master"])?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

/// Set the commit template
pub fn set_template(comment: &str) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(TEMPLATE_FILE)?;
    write!(&mut f, "{comment}")?;

    let output = git(&["config", "commit.template", ".git/issue.template"])?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

/// Clear the commit template
pub fn clear_template() -> Result<()> {
    remove_file(TEMPLATE_FILE)?;

    let output = git(&["config", "--unset", "commit.template"])?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

/// Call git
fn git(params: &[&str]) -> Result<Output> {
    let r = Command::new("git")
        .args(params)
        .output()
        .with_context(|| format!("cannot launch git {}", params[0]))?;
    Ok(r)
}
