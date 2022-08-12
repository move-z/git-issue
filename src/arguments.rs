use clap::{ArgGroup, Parser};

/// Command line arguments
#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(ArgGroup::new("pars").args(&["id", "config"]).required(true)))]
pub struct Args {
    /// The id of the issue
    #[clap(value_parser)]
    pub id: Option<String>,

    /// Set or change the configuration
    #[clap(long, action)]
    pub config: bool,

    /// Create a branch from the issue title
    #[clap(short, long, action)]
    pub branch: bool,
}
