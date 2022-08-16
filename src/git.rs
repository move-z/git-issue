const TEMPLATE_FILE: &str = ".git/issue.template";

use std::{
    fs::{remove_file, OpenOptions},
    io::Write,
    process::Command,
};

use anyhow::{bail, Context, Result};

/// Check that we are in a git repository
pub fn check_is_git() -> Result<bool> {
    Ok(git(&["status"])?.ok())
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

    match output {
        GitResult::Err(_) => bail!("cannot get property {full_name} from git"),
        GitResult::Ok(s) => Ok(s.trim().to_string()),
    }
}

/// Switch to branch, creating it if necessary
pub fn create_switch_branch(branch: &str) -> Result<()> {
    if !branch_exists(branch)? {
        new_branch(branch)?;
    }
    checkout(branch)
}

/// Create new branch
fn new_branch(branch: &str) -> Result<()> {
    let output = git(&["branch", branch])?;
    match output {
        GitResult::Err(e) => bail!(e),
        GitResult::Ok(_) => Ok(()),
    }
}

/// Check if a branch exists
fn branch_exists(branch: &str) -> Result<bool> {
    Ok(git(&["show-ref", &format!("refs/heads/{branch}")])?.ok())
}

/// Switch branch
pub fn checkout(branch: &str) -> Result<()> {
    let output = git(&["checkout", branch])?;
    match output {
        GitResult::Err(e) => bail!(e),
        GitResult::Ok(_) => Ok(()),
    }
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
    match output {
        GitResult::Err(e) => bail!(e),
        GitResult::Ok(_) => Ok(()),
    }
}

/// Clear the commit template
pub fn clear_template() -> Result<()> {
    remove_file(TEMPLATE_FILE)?;

    let output = git(&["config", "--unset", "commit.template"])?;
    match output {
        GitResult::Err(e) => bail!(e),
        GitResult::Ok(_) => Ok(()),
    }
}

/// Call git
fn git(params: &[&str]) -> Result<GitResult> {
    let r = Command::new("git")
        .args(params)
        .output()
        .with_context(|| format!("cannot launch git {}", params[0]))?;
    if r.status.success() {
        Ok(GitResult::Ok(String::from_utf8(r.stdout)?))
    } else {
        Ok(GitResult::Err(String::from_utf8(r.stderr)?))
    }
}

/// Possible git invocations results
enum GitResult {
    Ok(String),
    Err(String),
}

/// Helper functions
impl GitResult {
    fn ok(&self) -> bool {
        matches!(self, GitResult::Ok(_))
    }
}
