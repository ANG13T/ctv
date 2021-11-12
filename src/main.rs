use structopt::StructOpt;
use anyhow::{Context, Result};
use std::{env, fs};
use  std::fs::metadata;
// mod input;
// mod text_effects;
// mod utils;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
// use structopt::StructOpt;
use std::cmp::Ordering;

// struct Directory {
//     paths: Vec<File>,
//     args: input::Cli,
// }
  
// #[derive(Clone)]
// struct File {
//   path:      std::path::PathBuf,
//   file_type: Vec<PathType>,
//   group:     String,
//   user:      String,
//   modified:  String,
//   created:   String,
//   size:      String,
//   perms:     String,
// }

// fn printDirAndFilesForDir() -> Array<File|Directory>{

// }

// fn printFileString(input : File) -> String {
//     return "";
// }


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    path: std::path::PathBuf,
}

fn getDirName(path: String) -> String {
    let res: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
    return res.last().unwrap().to_string();
}

fn padValues(line: String, amount: i8) -> String{
    let mut newString = "".to_owned();
    let pad = "   ".to_owned();
    for i in 0..amount {
        newString.push_str(&pad);
    }
    newString.push_str(&line);
    return newString;
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

        // metadata.is_file
        if metadata.is_file(){
            println!("{value}", value=padValues("this is a file".to_string(), initialAmount - amount));
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")
            );
        }else if metadata.is_dir(){
            println!("{value} -> ", value=getDirName(entry.path().display().to_string()));
            return readdirLoop(entry.path(), amount - 1, initialAmount);
        }
    }

    Ok(())

}
