use structopt::StructOpt;
use anyhow::{Context, Result};
use std::{env, fs};
use  std::fs::metadata;
mod input;
mod decorators;
mod protocols;
mod services;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
// use structopt::StructOpt;
use std::cmp::Ordering;



/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    path: std::path::PathBuf,
}

// using Context from anyhow library to provide context for error messages.  it also keeps the original error, so we get a “chain” of error messages pointing out the root cause.
fn main() -> Result<()>{
    let current_dir = env::current_dir()?;
    let var = readdirLoop(current_dir, 3, 3);
    Ok(())
}

fn readdirLoop(dir: PathBuf, amount: i8, initialAmount: i8) -> Result<()>{
    if amount == 0 {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if metadata.is_file(){
            let coolFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount);
            print!("{:?}", coolFile);

        }else if metadata.is_dir(){
            let dirFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount);
            print!("{:?}", dirFile);
            return readdirLoop(entry.path(), amount - 1, initialAmount);
        }
    }

    Ok(())
}


#[cfg(test)]
mod test_suite;