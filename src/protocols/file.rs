use crate::services;
// use crate::input;
use crate::input;
use structopt::StructOpt;
use crate::decorators;
use crate::protocols::{PathType, colormanager};
// use structopt::StructOpt;

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
}

#[derive(Clone)]
pub struct FileStyle {
  size_position: i32,
  owner_position: i32,
  perms_position: i32, 
  dir_name_color: String,
  file_name_color: String,
  file_time_color: String,
  file_size_color: String,
  file_owner_color: String,
  file_perms_color: String,
  dir_name_style: String,
  file_name_style: String,
  file_size_style: String,
  file_owner_style: String,
  file_perms_style: String,
  file_time_style: String,
}
// TODO: change perms
impl FileStyle {
  pub fn new(
    size_pos: i32,
    owner_pos: i32,
    perms_pos: i32, 
    dir_name_col: String,
    file_name_col: String,
    file_time_col: String,
    file_size_col: String,
    file_owner_col: String,
    file_perms_col: String,
    dir_name_sty: String,
    file_name_sty: String,
    file_size_sty: String,
    file_owner_sty: String,
    file_perms_sty: String,
    file_time_sty: String) -> Self {
      Self {
        size_position: size_pos,
        owner_position: owner_pos,
        perms_position: perms_pos, 
        dir_name_color: dir_name_col,
        file_name_color: file_name_col,
        file_time_color: file_time_col,
        file_size_color: file_size_col,
        file_owner_color: file_owner_col,
        file_perms_color: file_perms_col,
        dir_name_style: dir_name_sty,
        file_name_style: file_name_sty,
        file_size_style: file_size_sty,
        file_owner_style: file_owner_sty,
        file_perms_style: file_perms_sty,
        file_time_style: file_time_sty
      }
    }
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
  perms:     String,
  styles:    FileStyle
}

impl File {
    pub fn new(file: std::path::PathBuf, time_format: String, styles: &FileStyle) -> Self {
      let ref_to_file_styles: FileStyle = styles.clone();
      Self {
        group:     services::group(file.to_path_buf()),
        user:      services::user(file.to_path_buf()),
        modified:  services::file_times::modified(file.to_path_buf(), time_format.to_owned()),
        created:   services::file_times::created(file.to_path_buf(), time_format),
        size:      services::size::size(file.to_path_buf()),
        perms:     services::perms::perms(file.to_path_buf()),
        file_type: PathType::new(&file).unwrap(),
        path: file,
        styles: ref_to_file_styles
      }
    }

    fn get_color_for(&self, typ: &str, input: String) -> String{
      let result = match typ {
        "FILE_OWNER_COLOR"=> colormanager::colorize_string(&self.styles.file_owner_color, input),
        "FILE_SIZE_COLOR"=> colormanager::colorize_string(&self.styles.file_size_color, input),
        "DIR_NAME_COLOR"=> colormanager::colorize_string(&self.styles.dir_name_color, input),
        "FILE_NAME_COLOR"=> colormanager::colorize_string(&self.styles.file_name_color, input),
        _=> "".to_string()
      };
      return result;
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
        let time: String = if input::Cli::from_args().created_time { self.created.to_string() } else { self.modified.to_string() };
        let file_size_color_string = format!("{}", self.size);
       let file_owner_color_string = format!("{}", self.user);
  
      return format!("{} [{file_size} {file_owner} {file_time} {}]",
       res, self.perms,
        file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
        file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
        file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style)
      );
    }

    pub fn get_styled_text(&self, text: &str, style: &str) -> String{
      let result = match style{
        "BOLD"=>decorators::bold(text),
        "DIMMED"=>decorators::dimmed(text),
        "ITALIC"=>decorators::italic(text),
        "UNDERLINE"=> decorators::underline(text),
        "BLINK"=>decorators::blink(text),
        "REVERSE"=>decorators::reverse(text),
        "HIDDEN"=>decorators::hidden(text),
        "STRICKEN"=>decorators::stricken(text),
        _=> decorators::bold("INVALID FONT STYLE"),
        };
        return result;
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
      
       // TODO: do timing stuff (env check if mod or created)
       let time: String = if input::Cli::from_args().created_time { self.created.to_string() } else { self.modified.to_string() };
       let file_size_color_string = format!("{}", self.size);
       let file_owner_color_string = format!("{}", self.user);
  
      return writeln!(f, "{} [{file_size} {file_owner} {file_time} {}]",
       res, self.perms,
        file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
        file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
        file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style)
      );

    }
  }
