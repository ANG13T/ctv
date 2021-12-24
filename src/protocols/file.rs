use crate::services;
// use crate::input;
use crate::protocols::{PathType};
// use structopt::StructOpt;

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
}


#[derive(Clone)]
pub struct File {
  path:      std::path::PathBuf,
  file_type: Vec<PathType>,
  group:     String,
  user:      String,
  modified:  String,
  created:   String,
  size:      String,
  perms:     String
}

impl File {
    pub fn new(file: std::path::PathBuf, time_format: String) -> Self {
      Self {
        group:     services::group(file.to_path_buf()),
        user:      services::user(file.to_path_buf()),
        modified:  services::file_times::modified(file.to_path_buf(), time_format.to_owned()),
        created:   services::file_times::created(file.to_path_buf(), time_format),
        size:      services::size::size(file.to_path_buf()),
        perms:     services::perms::perms(file.to_path_buf()),
        file_type: PathType::new(&file).unwrap(),
        path: file
      }
    }

    pub fn display_format(&self) -> String {
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

        // TODO: do timing stuff
        // let time = if input::Cli::from_args().created_time { &self.created } else { &self.modified };

  
      return format!("{} [{green}{} {yellow}{} {}]",
       res, self.size, self.user, self.perms,
        green = termion::color::Fg(termion::color::LightGreen),
        yellow = termion::color::Fg(termion::color::Yellow)
      );
    }

    fn get_sort_type(sort_t: [bool; 4]) -> DirSortType {
      for (i, t) in sort_t.iter().enumerate() {
        if *t {
          match i {
            0 => return DirSortType::Name,
            1 => return DirSortType::Created,
            2 => return DirSortType::Modified,
            3 => return DirSortType::Size,
            _ => (),
          }
        }
      }
      DirSortType::Not
    } 

    fn sort_directory_then_path(&mut self) {
      let new = &self.paths;
      let mut newer = Vec::new();
      let mut directories = Vec::new();
      for (i, f) in new.iter().enumerate() {
        if f.path.symlink_metadata().unwrap().is_dir() {
          directories.push(new[i].to_owned());
        } else {
          newer.push(new[i].to_owned())
        }
      }
  
      match get_sort_type([
        self.args.name,
        self.args.created,
        self.args.modified,
        self.args.size,
      ]) {
        DirSortType::Name => {
          name_sort(&mut directories);
          name_sort(&mut newer)
        }
        DirSortType::Created => {
          create_sort(&mut directories);
          create_sort(&mut newer)
        }
        DirSortType::Modified => {
          modified_sort(&mut directories);
          modified_sort(&mut newer)
        }
        DirSortType::Size => {
          size_sort(&mut directories);
          size_sort(&mut newer)
        }
        DirSortType::Not => (),
      }
  
      directories.append(&mut newer);
      self.paths = directories;
    }

    pub fn get_name(&self) -> String {
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
      return res;
    }
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
      
       // TODO: do timing stuff
       // let time = if input::Cli::from_args().created_time { &self.created } else { &self.modified };

  
      return writeln!(f, "{} [{green}{} {yellow}{} {}]",
       res, self.size, self.user, self.perms,
        green = termion::color::Fg(termion::color::LightGreen),
        yellow = termion::color::Fg(termion::color::Yellow)
      );

    }
  }
