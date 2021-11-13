use structopt::StructOpt;
use anyhow::{Context, Result};
use std::{env, fs};
use  std::fs::metadata;
mod input;
mod decorators;
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

impl PathType {
    fn new(file: &Path) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
      let mut return_val = Vec::new();
      if file.symlink_metadata()?.is_dir() {return_val.push(Self::Dir) }
      if file.symlink_metadata()?.file_type().is_symlink() {return_val.push(Self::Symlink)}
      if file.symlink_metadata()?.file_type().is_fifo() {return_val.push(Self::Pipe)}
      if file.symlink_metadata()?.file_type().is_char_device() {return_val.push(Self::CharD)}
      if file.symlink_metadata()?.file_type().is_block_device() {return_val.push(Self::BlockD)}
      if file.symlink_metadata()?.file_type().is_socket() {return_val.push(Self::Socket)}
      if return_val.is_empty() {return_val.push(Self::Path)}
  
      Ok(return_val)
    }
  
    fn create_letter(&self, letter: &str) -> String {
      format!(
        "{}{}{}{}",
        self.get_color_for_type(),
        letter,
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
      )
    }
  
    fn get_letter_for_type(&self) -> String {
      match self {
        Self::Dir     => self.create_letter("d"),
        Self::Symlink => self.create_letter("l"),
        Self::Pipe    => self.create_letter("|"),
        Self::CharD   => self.create_letter("c"),
        Self::BlockD  => self.create_letter("b"),
        Self::Socket  => self.create_letter("s"),
        _             => self.create_letter("."),
      }
    }
  
    fn get_color_for_type(&self) -> String {
      match self {
        Self::Dir     => format!("{}", termion::color::Fg(termion::color::LightBlue)),
        Self::Symlink => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
        Self::Path    => format!("{}", termion::color::Fg(termion::color::White)),
        Self::Pipe    => format!("{}", termion::color::Fg(termion::color::Yellow)),
        Self::CharD   => format!("{}{}", termion::color::Bg(termion::color::Yellow), termion::color::Fg(termion::color::LightBlue)),
        Self::BlockD  => format!("{}", termion::color::Fg(termion::color::LightGreen)),
        Self::Socket  => format!("{}", termion::color::Fg(termion::color::LightRed)),
      }
    }
  
    fn get_text_traits_for_type(&self, name: &str, file: &Path) -> String {
      match self {
        Self::Dir     => decorators::bold(&format!( "{}{}/", name, termion::color::Fg(termion::color::White))),
        Self::Symlink => decorators::italic(&format!( "{} -> {}", name, std::fs::read_link(file).unwrap().display().to_string())),
        Self::Path    => decorators::bold(name),
        Self::Pipe    => decorators::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
        Self::CharD   => decorators::bold(name),
        Self::BlockD  => decorators::bold(name),
        Self::Socket  => decorators::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
      }
    }
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

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let mut res = String::new();
      for (i, v) in self.file_type.iter().enumerate() {
        if i == 0 {
          res = format!(
            "{}{}",
            v.get_color_for_type(),
            v.get_text_traits_for_type(
              &self.path.
                components()
                .next_back()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
              &self.path
            )
          );
        } else {
          res = format!(
            "{}{}",
            v.get_color_for_type(),
            v.get_text_traits_for_type(&res, &self.path)
          );
        }
      }
      write!(f, "{}", res)
    }
  }

  impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let mut res = String::new();
      for (i, v) in self.file_type.iter().enumerate() {
        if i == 0 {
          res = v.get_text_traits_for_type(
            &self.path
              .components()
              .next_back()
              .unwrap()
              .as_os_str()
              .to_string_lossy()
              .to_string(),
            &self.path
          );
          res = format!("{}{}", v.get_color_for_type(), res);
          continue;
        }
        res = v.get_text_traits_for_type(&res, &self.path);
        res = format!("{}{}", v.get_color_for_type(), res);
      }
  
        let time = if input::Cli::from_args().created_time { &self.created } else { &self.modified };
  
      writeln!(f, "{} {green}{} {yellow}{} {blue} {}{} {}",
        self.perms, self.size, self.user, self.group, time, res,
        green = termion::color::Fg(termion::color::LightGreen),
        yellow = termion::color::Fg(termion::color::Yellow),
        blue = termion::color::Fg(termion::color::Blue),
      )
    }
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
            let coolFile = File::new(entry.path(), "".to_string());
            print!("{:?}", coolFile);
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
