use crate::services;
use crate::input;
use crate::protocols::{PathType};
use structopt::StructOpt;

#[derive(Clone)]
pub struct File {
  path:      std::path::PathBuf,
  file_type: Vec<PathType>,
  group:     String,
  user:      String,
  modified:  String,
  created:   String,
  size:      String,
  perms:     String,
  padding: i8
}

impl File {
    pub fn new(file: std::path::PathBuf, time_format: String, padding_amount: i8) -> Self {
      Self {
        group:     services::group(file.to_path_buf()),
        user:      services::user(file.to_path_buf()),
        modified:  services::file_times::modified(file.to_path_buf(), time_format.to_owned()),
        created:   services::file_times::created(file.to_path_buf(), time_format),
        size:      services::size::size(file.to_path_buf()),
        perms:     services::perms::perms(file.to_path_buf()),
        file_type: PathType::new(&file).unwrap(),
        path: file,
        padding: padding_amount
      }
    }

    fn getPaddingString(&self) -> String {
        let mut newString = "".to_owned();
        if self.padding == 0{
          newString = "".to_owned();
        }
        let pad = " ".to_owned();
        for i in 0..self.padding {
          newString.push_str(&pad);
        }

        if self.padding > 0{
            newString.push_str("⎿__");
        }
        
        return newString;
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
  
        let time = if input::Cli::from_args().created_time { &self.created } else { &self.modified };
  
      return writeln!(f, "{padding}{} [{green}{} {yellow}{} {} {}]",
       res, self.size, self.user, self.perms, time,
        green = termion::color::Fg(termion::color::LightGreen),
        yellow = termion::color::Fg(termion::color::Yellow),
        padding = self.getPaddingString()
      );

    }
  }
