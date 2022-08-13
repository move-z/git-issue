use anyhow::{bail, Result};
use clap::{ArgGroup, Parser};

use crate::git::*;

mod git;
mod github;
mod jira;
mod utils;

fn main() -> Result<()> {
    if !check_is_git()? {
        bail!("this is not a git repository");
    }

    let args = Args::parse();

    let kind = get_config("personality")?;

    if args.clear {
        clear()?;
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

    let branch = format!("{id}-{}", title.replace(' ', "-"));
    let comment = format!("{id} - {title}");

    if do_branch {
        create_switch_branch(&branch)?;
    }

    set_template(&comment)?;

    Ok(())
}

/// Cleanup
fn clear() -> Result<()> {
    clear_branch()?;
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
pub struct Args {
    /// The id of the issue
    #[clap(value_parser)]
    pub id: Option<String>,

    /// Clear comment template and eventually go back with the branch
    #[clap(short, long, action)]
    pub clear: bool,

    /// Create a branch from the issue title
    #[clap(short, long, action)]
    pub branch: bool,
}
