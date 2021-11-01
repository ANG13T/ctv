use structopt::StructOpt;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// using Context from anyhow library to provide context for error messages.  it also keeps the original error, so we get a “chain” of error messages pointing out the root cause.

fn main() -> Result<()>{
    // read file from string
    // question mark at end automatically returns an error if path is incorrect
    let path = "test.txt";
    let result = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
