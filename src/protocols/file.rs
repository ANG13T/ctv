use crate::services;
// use crate::input;
use crate::input;
use std::{fs, env};
use structopt::StructOpt;
use crate::decorators;
use crate::protocols::{PathType, colormanager, EnvManager};
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
  file_extension_color: String,
  dir_name_style: String,
  file_name_style: String,
  file_size_style: String,
  file_owner_style: String,
  file_perms_style: String,
  file_time_style: String,
  file_extension_style: String
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
    file_ext_col: String,
    dir_name_sty: String,
    file_name_sty: String,
    file_size_sty: String,
    file_owner_sty: String,
    file_perms_sty: String,
    file_time_sty: String,
    file_ext_sty: String) -> Self {
      Self {
        size_position: size_pos,
        owner_position: owner_pos,
        perms_position: perms_pos, 
        dir_name_color: dir_name_col.to_uppercase(),
        file_name_color: file_name_col.to_uppercase(),
        file_time_color: file_time_col.to_uppercase(),
        file_size_color: file_size_col.to_uppercase(),
        file_owner_color: file_owner_col.to_uppercase(),
        file_perms_color: file_perms_col.to_uppercase(),
        file_extension_color: file_ext_col.to_uppercase(),
        dir_name_style: dir_name_sty.to_uppercase(),
        file_name_style: file_name_sty.to_uppercase(),
        file_size_style: file_size_sty.to_uppercase(),
        file_owner_style: file_owner_sty.to_uppercase(),
        file_perms_style: file_perms_sty.to_uppercase(),
        file_time_style: file_time_sty.to_uppercase(),
        file_extension_style: file_ext_sty.to_uppercase()
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
  accessed:  String,
  created:   String,
  size:      String,
  perms:     String,
  styles:    FileStyle,
  file_time_type: String,
  show_extension: bool,
  env_manager: EnvManager
}

impl File {
  // TODO: add diff time options
    pub fn new(file: std::path::PathBuf, time_format: &str, time_type: &str, styles: &FileStyle, show_ext: bool, env_manager: EnvManager) -> Self {
      let ref_to_file_styles: FileStyle = styles.clone();      
      
      Self {
        group:     services::group(file.to_path_buf()),
        user:      services::user(file.to_path_buf()),
        modified:  services::time::time_modified(file.to_path_buf(), time_format),
        created:   services::time::time_created(file.to_path_buf(), time_format),
        accessed:  services::time::time_acessed(file.to_path_buf(), time_format),
        size:      services::size::size(file.to_path_buf()),
        perms:     services::perms::perms(file.to_path_buf()),
        file_type: PathType::new(&file).unwrap(),
        path: file,
        styles: ref_to_file_styles,
        file_time_type: time_type.to_string(),
        show_extension: show_ext,
        env_manager: env_manager
      }
    }

    fn get_color_for(&self, typ: &str, input: String) -> String{
      let result = match typ {
        "FILE_OWNER_COLOR"=> colormanager::colorize_string(&self.styles.file_owner_color, input),
        "FILE_SIZE_COLOR"=> colormanager::colorize_string(&self.styles.file_size_color, input),
        "DIR_NAME_COLOR"=> colormanager::colorize_string(&self.styles.dir_name_color, input),
        "FILE_NAME_COLOR"=> colormanager::colorize_string(&self.styles.file_name_color, input),
        "FILE_TIME_COLOR"=> colormanager::colorize_string(&self.styles.file_time_color, input),
        "FILE_EXTENSION_COLOR"=> colormanager::colorize_string(&self.styles.file_extension_color, input),
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

        // TODO: do timing stuff, edit below time typing
        let time: String = if self.file_time_type == "CREATED" { self.created.to_string() } else if self.file_time_type == "MODIFIED" { self.modified.to_string() } else {self.accessed.to_string()};
        let file_size_color_string = format!("{}", self.size);
       let file_owner_color_string = format!("{}", self.user);
       let metadata = fs::metadata(&self.path).unwrap();
       if metadata.is_dir(){
        let file_count = fs::read_dir(&self.path).unwrap().count();
        return format!( "{} [{file_size} {file_owner} {file_time} {}] ({} items)",
        res, self.perms, file_count,
         file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
         file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
         file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style)
       );
       }else {
         let file_ext = services::extension::extension(&self.path);
        return format!("{} [{file_size} {file_owner} {file_time} {file_extension} {}]",
        res, self.perms,
          file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
          file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
          file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style),
          file_extension = self.get_styled_text(&self.get_color_for("FILE_EXTENSION_COLOR", file_ext), &self.styles.file_extension_style)
        );
       }
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

  fn get_position_category(index: i32) -> String{
    if index == -1 {
      return "".to_string();
    }
    
    if self.env_manager.
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
       let time: String = if self.file_time_type == "CREATED" { self.created.to_string() } else { self.modified.to_string() };
       let file_size_color_string = format!("{}", self.size);
       let file_owner_color_string = format!("{}", self.user);
  
      let metadata = fs::metadata(&self.path).unwrap();
      if metadata.is_dir(){
        let file_count = fs::read_dir(&self.path).unwrap().count();
        return writeln!(f, "{} [{file_size} {file_owner} {file_time} {}] ({} items)",
        res, self.perms, file_count,
         file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
         file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
         file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style)
       );
      }else{
        let file_ext = services::extension::extension(&self.path);
        return writeln!(f, "{} [{file_size} {file_owner} {file_time} {file_extension} {}]",
        res, self.perms,
         file_size = self.get_styled_text(&self.get_color_for("FILE_SIZE_COLOR", file_size_color_string), &self.styles.file_size_style),
         file_owner = self.get_styled_text(&self.get_color_for("FILE_OWNER_COLOR", file_owner_color_string), &self.styles.file_owner_style),
         file_time = self.get_styled_text(&self.get_color_for("FILE_TIME_COLOR", time), &self.styles.file_time_style),
         file_extension = self.get_styled_text(&self.get_color_for("FILE_EXTENSION_COLOR", file_ext), &self.styles.file_extension_style)
       );
      }

    }
  }
