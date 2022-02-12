use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(default_value = ".")]
    pub dir: PathBuf,

    /// If provided, hide metadata and only show entry names
    #[clap(long, short = 'h')]
    pub short: bool,

    /// The maximum depth of the tree
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// Show config variables and exit
    #[clap(long = "config", short)]
    pub print_config: bool,
}

pub fn parse() -> anyhow::Result<Args> {
    Ok(Args::try_parse()?)
}
