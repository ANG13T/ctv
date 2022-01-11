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
            match key {
                "FILE_SIZE_POSITION" => self.file_size_position = val,
                "FILE_OWNER_POSITION" => self.file_owner_position = val,
                "FILE_PERMS_POSITION" => self.file_perms_position = val,
                "FILE_TIME_POSITION" => self.file_time_position = val,
                "FILE_EXTENSION_POSITION" => self.file_extension_position = val,
                "DIR_NAME_COLOR" => self.dir_name_color = val,
                "FILE_NAME_COLOR" => self.file_name_color = val,
                "FILE_TIME_COLOR" => self.file_time_color = val,
                "FILE_SIZE_COLOR" => self.file_size_color = val,
                "FILE_OWNER_COLOR" => self.file_owner_color = val,
                "FILE_PERMS_COLOR" => self.file_perms_color = val,
                "FILE_EXTENSION_COLOR" => self.file_extension_color = val,
                "FILE_TIME_FORMAT" => self.file_time_format = val,
                "FILE_TIME_TYPE" => self.file_time_type = val,
                "TREE_LAYER_LIMIT" => self.tree_layer_limit = val,
                "SHOW_FILE_METADATA" => self.show_file_metadata = val,
                "SHOW_DIR_METADATA" => self.show_dir_metadata = val,
                "PIPE" => self.pipe = val,
                "ELBOW" => self.elbow = val,
                "TEE" => self.tee = val,
                "PIPE_PREFIX" => self.pipe_prefix = val,
                "SPACE_PREFIX" => self.space_prefix = val,
                "DIR_NAME_STYLE" => self.dir_name_style = val,
                "FILE_NAME_STYLE" => self.file_name_style = val,
                "FILE_TIME_STYLE" => self.file_time_style = val,
                "FILE_SIZE_STYLE" => self.file_size_style = val,
                "FILE_OWNER_STYLE" => self.file_owner_style = val,
                "FILE_PERMS_STYLE" => self.file_perms_style = val,
                "FILE_EXTENSION_STYLE" => self.file_extension_style = val,
            }
        }
    }
}