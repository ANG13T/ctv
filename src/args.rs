use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(default_value = ".")]
    pub dir: PathBuf,

    #[clap(long, short = 'h')]
    pub short: bool,

    #[clap(short, long)]
    pub limit: Option<usize>,

    /// Show config variables and exit
    #[clap(long = "config", short)]
    pub print_config: bool,
}

pub fn parse() -> anyhow::Result<Args> {
    Ok(Args::try_parse()?)
}
