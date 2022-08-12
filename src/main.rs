use anyhow::{bail, Result};
use clap::Parser;

use arguments::Args;
use utils::get_config;

mod arguments;
mod github;
mod jira;
mod utils;

fn main() -> Result<()> {
    let args = Args::parse();
    let id = args.id;

    let kind = get_config("personality")?;

    let issue = match kind.as_str() {
        "github" => github::get_issue_title(&id)?,
        "jira" => jira::get_issue_title(&id)?,
        _ => bail!("{kind} is not a valid value for personality\nvalid values: github, jira"),
    };

    dbg!(issue);

    Ok(())
}
