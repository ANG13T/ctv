use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Cli {
  /// Give me a directory
  #[structopt(parse(from_os_str), default_value = ".")]
  pub dir: std::path::PathBuf,

  /// Sorting by name
  #[structopt(short = "n", long = "name")]
  pub name: bool,

  /// Sorting by created date
  #[structopt(short = "c", long = "created")]
  pub created: bool,

  /// Sorting by modified date
  #[structopt(short = "m", long = "modified")]
  pub modified: bool,

  /// Sorting by size
  #[structopt(short = "s", long = "size")]
  pub size: bool,

  /// Uses short format (only name)
  #[structopt(short = "s", long = "short")]
  pub short: bool,

  /// Sets layer limit
  #[structopt(short = "l", long = "layer")]
  pub layer: i32,

  /// Uses specific time format
  #[structopt(long = "time-format", default_value = "%e %b %H.%M")]
  pub time_format: String,

  /// Shows the file created time instead of the file modified time
  #[structopt(short = "i", long = "ct")]
  pub created_time: bool,
}