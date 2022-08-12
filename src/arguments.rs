use clap::Parser;

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
