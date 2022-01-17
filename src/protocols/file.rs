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
  pub positions: DisplayPositions,
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
  file_extension_style: String,
  num_positions: i32
}
// TODO: change perms
impl FileStyle {
  pub fn new(
    display_pos: DisplayPositions,
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
    file_ext_sty: String,
    file_num_pos: i32) -> Self {
      Self {
        positions: display_pos, 
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
        file_extension_style: file_ext_sty.to_uppercase(),
        num_positions: file_num_pos
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
  display_positions: DisplayPositions
}

#[derive(Clone)]
pub struct DisplayPositions {
  file_size_position: i32,
  file_owner_position: i32,
  file_perms_position: i32,
  file_time_position: i32,
  file_extension_position: i32
}

impl DisplayPositions {
  pub fn new(file_size_pos: i32, file_owner_pos: i32, file_perms_pos: i32, file_time_pos: i32, file_extension_pos: i32) -> Self{
    Self {
      file_extension_position: file_extension_pos,
      file_time_position: file_time_pos,
      file_perms_position: file_perms_pos, 
      file_owner_position: file_owner_pos,
      file_size_position: file_size_pos
    }
  }
}

impl File {
  // TODO: add diff time options
    pub fn new(file: std::path::PathBuf, time_format: &str, time_type: &str, styles: &FileStyle, show_ext: bool, display_pos: &DisplayPositions) -> Self {
      let ref_to_file_styles: FileStyle = styles.clone();    
      let ref_to_display_position: DisplayPositions = display_pos.clone();      
      
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
        display_positions: ref_to_display_position
      }
    }

    fn get_color_for(&self, typ: &str, input: String) -> String{
      let result = match typ {
        "FILE_OWNER_POSITION"=> colormanager::colorize_string(&self.styles.file_owner_color, input),
        "FILE_SIZE_POSITION"=> colormanager::colorize_string(&self.styles.file_size_color, input),
        "DIR_NAME_POSITION"=> colormanager::colorize_string(&self.styles.dir_name_color, input),
        "FILE_NAME_POSITION"=> colormanager::colorize_string(&self.styles.file_name_color, input),
        "FILE_TIME_POSITION"=> colormanager::colorize_string(&self.styles.file_time_color, input),
        "FILE_EXTENSION_POSITION"=> colormanager::colorize_string(&self.styles.file_extension_color, input),
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

       let metadata = fs::metadata(&self.path).unwrap();
       if metadata.is_dir(){
        let file_count = fs::read_dir(&self.path).unwrap().count();
        return format!( "{} [{element_one}{element_two}{element_three}{element_four}] ({} items)",
        res, file_count,
        element_one = self.get_styled_text(&self.get_color_for(&self.get_position_category(1), self.get_result_for_position(&self.get_position_category(1))), &self.get_style_for_position(&self.get_position_category(1)), 1 == self.styles.num_positions),
        element_two = self.get_styled_text(&self.get_color_for(&self.get_position_category(2), self.get_result_for_position(&self.get_position_category(2))), &self.get_style_for_position(&self.get_position_category(2)), 2 == self.styles.num_positions),
        element_three = self.get_styled_text(&self.get_color_for(&self.get_position_category(3), self.get_result_for_position(&self.get_position_category(3))), &self.get_style_for_position(&self.get_position_category(3)), 3 == self.styles.num_positions),
        element_four = self.perms
       );
       }else {
        return format!("{} [{element_one}{element_two}{element_three}{element_four}{element_five}]",
        res, 
        element_one = self.get_styled_text(&self.get_color_for(&self.get_position_category(1), self.get_result_for_position(&self.get_position_category(1))), &self.get_style_for_position(&self.get_position_category(1)), 1 == self.styles.num_positions),
        element_two = self.get_styled_text(&self.get_color_for(&self.get_position_category(2), self.get_result_for_position(&self.get_position_category(2))), &self.get_style_for_position(&self.get_position_category(2)), 2 == self.styles.num_positions),
        element_three = self.get_styled_text(&self.get_color_for(&self.get_position_category(3), self.get_result_for_position(&self.get_position_category(3))), &self.get_style_for_position(&self.get_position_category(3)), 3 == self.styles.num_positions),
        element_four = self.get_styled_text(&self.get_color_for(&self.get_position_category(4), self.get_result_for_position(&self.get_position_category(4))), &self.get_style_for_position(&self.get_position_category(4)), 4 == self.styles.num_positions),
        element_five = self.get_styled_text(&self.get_color_for(&self.get_position_category(5), self.get_result_for_position(&self.get_position_category(5))), &self.get_style_for_position(&self.get_position_category(5)), 5 == self.styles.num_positions)
        );
       }
    }

    pub fn get_styled_text(&self, text: &str, style: &str, is_end: bool) -> String{
      if text.len() == 0 {return "".to_string()};
      let mut result = match style{
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
        if !is_end {
          result.push_str(" ");
        }
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

    pub fn get_position_category(&self, index: i32) -> String{
    
      if self.display_positions.file_size_position == index {
        return "FILE_SIZE_POSITION".to_string();
      }

      if self.display_positions.file_owner_position == index {
        return "FILE_OWNER_POSITION".to_string();
      }

      if self.display_positions.file_perms_position == index {
        return "FILE_PERMS_POSITION".to_string();
      }

      if self.display_positions.file_time_position == index {
        return "FILE_TIME_POSITION".to_string();
      }

      if self.display_positions.file_extension_position == index {
        return "FILE_EXTENSION_POSITION".to_string();
      }
  
      return "".to_string();
    }

    // TODO: do timing stuff (env check if mod or created or accessed)
    pub fn get_result_for_position(&self, position: &str) -> String{
      let res = match position {
        "FILE_SIZE_POSITION" => format!("{}", self.size),
        "FILE_OWNER_POSITION" => format!("{}", self.user),
        "FILE_PERMS_POSITION" => format!("{}", self.perms),
        "FILE_TIME_POSITION" => {
          let time: String = if self.file_time_type == "CREATED" { self.created.to_string() } else { self.modified.to_string() };
          return time
        },
        "FILE_EXTENSION_POSITION" => services::extension::extension(&self.path),
        _=>"".to_string()
      };
      return res;
    }

    pub fn get_style_for_position(&self, position: &str) -> String {
      return match position {
        "FILE_SIZE_POSITION" => self.styles.file_size_style.clone(),
        "FILE_OWNER_POSITION" => self.styles.file_owner_style.clone(),
        "FILE_PERMS_POSITION" => self.styles.file_perms_style.clone(),
        "FILE_TIME_POSITION" => self.styles.file_time_style.clone(),
        "FILE_EXTENSION_POSITION" => self.styles.file_extension_style.clone(),
        _=>"".to_string()
      }
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
  
      let metadata = fs::metadata(&self.path).unwrap();
      if metadata.is_dir(){
        let file_count = fs::read_dir(&self.path).unwrap().count();
        return writeln!(f, "{} [{element_one}{element_two}{element_three}{element_four}] ({} items)",
        res, file_count,
         element_one = self.get_styled_text(&self.get_color_for(&self.get_position_category(1), self.get_result_for_position(&self.get_position_category(1))), &self.get_style_for_position(&self.get_position_category(1)), 1 == self.styles.num_positions),
         element_two = self.get_styled_text(&self.get_color_for(&self.get_position_category(2), self.get_result_for_position(&self.get_position_category(2))), &self.get_style_for_position(&self.get_position_category(2)), 2 == self.styles.num_positions),
         element_three = self.get_styled_text(&self.get_color_for(&self.get_position_category(3), self.get_result_for_position(&self.get_position_category(3))), &self.get_style_for_position(&self.get_position_category(3)), 3 == self.styles.num_positions),
         element_four = self.perms
       );
      }else{
        return writeln!(f, "{} [{element_one}{element_two}{element_three}{element_four}{element_five}]",
        res, 
         element_one = self.get_styled_text(&self.get_color_for(&self.get_position_category(1), self.get_result_for_position(&self.get_position_category(1))), &self.get_style_for_position(&self.get_position_category(1)), 1 == self.styles.num_positions),
         element_two = self.get_styled_text(&self.get_color_for(&self.get_position_category(2), self.get_result_for_position(&self.get_position_category(2))), &self.get_style_for_position(&self.get_position_category(2)), 2 == self.styles.num_positions),
         element_three = self.get_styled_text(&self.get_color_for(&self.get_position_category(3), self.get_result_for_position(&self.get_position_category(3))), &self.get_style_for_position(&self.get_position_category(3)), 3 == self.styles.num_positions),
         element_four = self.get_styled_text(&self.get_color_for(&self.get_position_category(4), self.get_result_for_position(&self.get_position_category(4))), &self.get_style_for_position(&self.get_position_category(4)), 4 == self.styles.num_positions),
         element_five = self.get_styled_text(&self.get_color_for(&self.get_position_category(5), self.get_result_for_position(&self.get_position_category(5))), &self.get_style_for_position(&self.get_position_category(5)), 5 == self.styles.num_positions)
       );
      }

    }
  }
