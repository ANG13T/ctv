use std::path::PathBuf;
use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Cli {
    /// Pass in a directory
    #[structopt(parse(from_os_str), default_value = ".")]
    pub dir: PathBuf,

    /// Uses short format
    #[structopt(short = "h", long = "short")]
    pub short: bool,

    /// Sets the tree layer limit
    #[structopt(short = "l", long = "layer", default_value = "3")]
    pub layer: String,

    /// Set config variable via CLI
    #[structopt(short = "s", long = "set-var", default_value = "")]
    pub set_var: String,

    /// Shows the file created time instead of the file modified time
    #[structopt(short = "i", long = "ct")]
    pub created_time: bool,

    /// Show all config variables
    #[structopt(short = "c", long = "config")]
    pub show_env: bool,
}

// TODO: add sorting and specific time format

// /// Uses specific time format
// #[structopt(long = "time-format", default_value = "%e %b %H.%M")]
// pub time_format: String,

// /// Sorting by name
// #[structopt(short = "n", long = "name")]
// pub name: bool,

// /// Sorting by created date
// #[structopt(short = "c", long = "created")]
// pub created: bool,

// /// Sorting by modified date
// #[structopt(short = "m", long = "modified")]
// pub modified: bool,

// /// Sorting by size
// #[structopt(short = "s", long = "size")]
// pub size: bool,
