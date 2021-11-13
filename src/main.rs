use structopt::StructOpt;
use anyhow::{Context, Result};
use std::{env, fs};
use  std::fs::metadata;
// mod input;
// mod text_effects;
mod services;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
// use structopt::StructOpt;
use std::cmp::Ordering;

struct Directory {
    paths: Vec<File>,
    // args: input::Cli,
}

#[derive(Copy, Clone, Debug)]
enum PathType {
  Dir,
  Symlink,
  Path,
  Pipe,
  CharD,
  BlockD,
  Socket,
}
  
#[derive(Clone)]
struct File {
  path:      std::path::PathBuf,
  file_type: Vec<PathType>,
  group:     String,
  user:      String,
  modified:  String,
  created:   String,
  size:      String,
  perms:     String,
}

impl File {
    fn new(file: std::path::PathBuf, time_format: String) -> Self {
      Self {
        group:     services::group(file.to_path_buf()),
        user:      services::user(file.to_path_buf()),
        modified:  services::file_times::modified(file.to_path_buf(), time_format.to_owned()),
        created:   services::file_times::created(file.to_path_buf(), time_format),
        size:      services::size::size(file.to_path_buf()),
        perms:     services::perms::perms(file.to_path_buf()),
        file_type: PathType::new(&file).unwrap(),
        path: file,
      }
    }
  }

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
            let formattedFilePath: String = format!("Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}", last_modified,
            metadata.permissions().readonly(),
            metadata.len(),
            path.file_name().ok_or("No filename"));

            println!("{value}", value=padValues(formattedFilePath.to_string(), initialAmount - amount));

        }else if metadata.is_dir(){
            println!("{value} -> ", value=getDirName(entry.path().display().to_string()));
            return readdirLoop(entry.path(), amount - 1, initialAmount);
        }
    }

    Ok(())

}
