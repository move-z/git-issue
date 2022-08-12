use std::fmt::format;

use anyhow::{bail, Result};
use clap::Parser;

use crate::git::{check_is_git, create_branch, get_config, set_template};

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

    let id = args.id;
    let title = match kind.as_str() {
        "github" => github::get_issue_title(&id)?,
        "jira" => jira::get_issue_title(&id)?,
        _ => bail!("{kind} is not a valid value for personality\nvalid values: github, jira"),
    };

    let branch = format!("{id}-{}", title.replace(" ", "-"));
    let comment = format!("{id} - {title}");

    if args.branch {
        create_branch(&branch)?;
    }

    set_template(&comment)?;

    Ok(())
}

/// Command line arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The id of the issue
    #[clap(value_parser)]
    pub id: String,

    /// Create a branch from the issue title
    #[clap(short, long, action)]
    pub branch: bool,
}
