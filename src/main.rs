use crate::arguments::Args;
use anyhow::Result;
use clap::Parser;

mod arguments;
mod git;
mod jira;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.config {
        eprintln!("config is TODO!!!");
    } else if let Some(id) = args.id {
        jira::handle(&id)?;
    }

    Ok(())
}
