use anyhow::{bail, Result};
use clap::{ArgGroup, Parser};
use regex::Regex;

use crate::git::*;

mod git;
mod github;
mod jira;
mod utils;

fn main() -> Result<()> {
    let args = Args::parse();

    if !check_is_git()? {
        bail!("this is not a git repository");
    }

    let kind = get_config("personality")?;

    if args.clear {
        clear(args.to)?;
    } else if let Some(id) = args.id {
        setup(&kind, &id, args.branch)?;
    }

    Ok(())
}

/// Setup issue <id>
fn setup(kind: &str, id: &str, do_branch: bool) -> Result<()> {
    let title = match kind {
        "github" => github::get_issue_title(id)?,
        "jira" => jira::get_issue_title(id)?,
        _ => bail!("{kind} is not a valid value for personality\nvalid values: github, jira"),
    };

    // partially implemented according to
    // https://git-scm.com/docs/git-check-ref-format
    let title = title
        .replace(char::is_whitespace, "-")
        .replace('/', "-")
        .replace(char::is_control, "")
        .replace(['~', '^', ':', '?', '*', '[', ']', '\\', '@', '{', '}'], "");
    let title = Regex::new(r"\.\.+")?.replace_all(&title, "");
    let title = Regex::new(r"--+")?.replace_all(&title, "-");

    let branch = format!("{id}-{}", title);
    let comment = format!("{id} - {title}");

    if do_branch {
        create_switch_branch(&branch)?;
    }

    set_template(&comment)?;

    Ok(())
}

/// Cleanup
fn clear(destination_branch: Option<String>) -> Result<()> {
    let default_branch = get_config("defaultbranch");
    let default_branch = default_branch.as_deref().unwrap_or("master");
    checkout(destination_branch.as_deref().unwrap_or(default_branch))?;
    clear_template()?;
    Ok(())
}

/// Command line arguments
#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(
            ArgGroup::new("args")
                .required(true)
                .args(&["id", "clear"]),
        ))]
#[clap(group(
            ArgGroup::new("require_clear")
                .requires("clear")
                .conflicts_with("id")
                .args(&["to"]),
        ))]
pub struct Args {
    /// The id of the issue
    #[clap(value_parser)]
    pub id: Option<String>,

    /// Create a branch from the issue title
    #[clap(short, long, action)]
    pub branch: bool,

    /// Clear comment template and eventually go back with the branch
    #[clap(short, long, action)]
    pub clear: bool,

    /// Specific branch to change to on clear
    #[clap(long, action)]
    pub to: Option<String>,
}
