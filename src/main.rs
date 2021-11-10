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

// using Context from anyhow library to provide context for error messages.  it also keeps the original error, so we get a “chain” of error messages pointing out the root cause.
fn main() -> Result<()>{
    // let current_dir = env::current_dir()?;

    // for entry in fs::read_dir(current_dir)? {
    //     let entry = entry?;
    //     let path = entry.path();

    //     let metadata = fs::metadata(&path)?;
    //     let last_modified = metadata.modified()?.elapsed()?.as_secs();

    //     // metadata.is_file
    //     if metadata.is_file(){

    //     }else if metadata.is_dir(){

    //     }

    //     println!(
    //         "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
    //         last_modified,
    //         metadata.permissions().readonly(),
    //         metadata.len(),
    //         path.file_name().ok_or("No filename")
    //     );
    // }

    Ok(())
}

fn readdirLoop(dir: PathBuf, amount: i8) -> Result<()>{
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
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")
            );
        }else if metadata.is_dir(){
            return readdirLoop(entry.path(), amount - 1);
        }
    }

    Ok(())

}
