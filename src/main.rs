use anyhow::{Result, bail};
use clap::Parser;

use arguments::Args;

mod arguments;
mod git;
mod github;
mod jira;

fn main() -> Result<()> {
    let args = Args::parse();
    let id = args.id;

    let kind = git::get_config("personality")?;

    let issue = match kind.as_str() {
        "github" => github::get_issue_title(&id)?,
        "jira" => jira::get_issue_title(&id)?,
        _ => bail!("{kind} is not a valid value for personality\nvalid values: github, jira"),
    };

    dbg!(issue);

    Ok(())
}
