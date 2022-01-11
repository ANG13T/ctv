extern crate dotenv;
use dotenv::dotenv;

#[derive(Clone)]
pub struct EnvManager {
  file_size_position: i32, 
  file_owner_position: i32, 
  file_perms_position: i32, 
  file_time_position: i32, 
  file_extension_position: i32, 
  dir_name_color: String,
  file_name_color: String,
  file_time_color: String,
  file_size_color: String,
  file_owner_color: String,
  file_perms_color: String,
  file_extension_color: String,
  dir_name_style: String,
  file_name_style: String,
  file_time_style: String,
  file_size_style: String,
  file_owner_style: String,
  file_perms_style: String,
  file_extension_style: String,
  file_time_format: String,
  file_time_type String,
  tree_layer_limit: i32, 
  show_file_metadata: bool,
  show_dir_metadata: bool,
  pipe: String,
  elbow: String,
  tee: String,
  pipe_prefix: String,
  space_prefix: String
}


impl EnvManager {
    pub fn new() -> {
        dotenv().ok();
        for (key, val) in env::vars() {
            
        }
    }
}