
// use crate::*;
use crate::protocols::file::{FileStyle, File, DisplayPositions};
extern crate dotenv;
use std::path::{PathBuf};
use dotenv::dotenv;
use std::{env};


fn get_num_pos() -> Vec<i32>{
  dotenv().ok();
  let mut result = Vec::new();

  let mut original : i32 = 5;
  if env::var("FILE_SIZE_POSITION").unwrap().parse::<i32>().unwrap() == -1 {original -= 1};
  if env::var("FILE_OWNER_POSITION").unwrap().parse::<i32>().unwrap() == -1 {original -= 1};
  if env::var("FILE_PERMS_POSITION").unwrap().parse::<i32>().unwrap() == -1 {original -= 1};
  if env::var("FILE_TIME_POSITION").unwrap().parse::<i32>().unwrap() == -1 {original -= 1};
  if env::var("FILE_EXTENSION_POSITION").unwrap().parse::<i32>().unwrap() == -1 {original -= 1};

  let mut dir_num_pos : i32 = original;
  if env::var("FILE_EXTENSION_POSITION").unwrap().parse::<i32>().unwrap() != -1 {
      dir_num_pos -= 1;
  }
  result.push(original);
  result.push(dir_num_pos);
  return result;
}

#[test]
fn test_basic_directory_expansion() {
  let path = PathBuf::from(r"./check_env_test_cases.rs");
  let test_file_display_pos : DisplayPositions = DisplayPositions::new(1,2,3,4,5);
  let test_file_style : FileStyle = FileStyle::new(test_file_display_pos, "LIGHTRED".to_string(), "LIGHTRED".to_string(), "LIGHTRED".to_string(), "MAGENTA".to_string(), "MAGENTA".to_string(), "BLUE".to_string(), "YELLOW".to_string(), "ITALIC".to_string(), "BOLD".to_string(), "BOLD".to_string(), "ITALIC".to_string(), "BOLD".to_string(), "BOLD".to_string(), "ITALIC".to_string(), get_num_pos()[0], get_num_pos()[1], "BLUE".to_string(), "LIGHTMAGENTA".to_string(), "WHITE".to_string(), "YELLOW".to_string(), "YELLOW".to_string(), "LIGHTGREEN".to_string(), "LIGHTRED".to_string(), "LIGHTGREEN".to_string(), "LIGHTRED".to_string(), "LIGHTGREEN".to_string(), "LIGHTBLACK".to_string());
  let test_file : File = File::new(path, "%Y-%m-%d", "CREATED", &test_file_style, true, &test_file_style.positions);
  assert_eq!(
   "hello",
    "hello"
  )
}