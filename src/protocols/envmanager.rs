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
  file_time_type: String,
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
    pub fn new() -> Self {
        dotenv().ok();

        let result = Self {
            file_size_position: 0, 
            file_owner_position: 0, 
            file_perms_position: 0, 
            file_time_position: 0, 
            file_extension_position: 0, 
            dir_name_color: '',
            file_name_color: '',
            file_time_color: '',
            file_size_color: '',
            file_owner_color: '',
            file_perms_color: '',
            file_extension_color: '',
            dir_name_style: '',
            file_name_style: '',
            file_time_style: '',
            file_size_style: '',
            file_owner_style: '',
            file_perms_style: '',
            file_extension_style: '',
            file_time_format: '',
            file_time_type '',
            tree_layer_limit: 0, 
            show_file_metadata: true,
            show_dir_metadata: true,
            pipe: '',
            elbow: '',
            tee: '',
            pipe_prefix: '',
            space_prefix: ''
        }

        for (key, val) in env::vars() {
            match key {
                "FILE_SIZE_POSITION" => result.file_size_position = val,
                "FILE_OWNER_POSITION" => result.file_owner_position = val,
                "FILE_PERMS_POSITION" => result.file_perms_position = val,
                "FILE_TIME_POSITION" => result.file_time_position = val,
                "FILE_EXTENSION_POSITION" => result.file_extension_position = val,
                "DIR_NAME_COLOR" => result.dir_name_color = val,
                "FILE_NAME_COLOR" => result.file_name_color = val,
                "FILE_TIME_COLOR" => result.file_time_color = val,
                "FILE_SIZE_COLOR" => result.file_size_color = val,
                "FILE_OWNER_COLOR" => result.file_owner_color = val,
                "FILE_PERMS_COLOR" => result.file_perms_color = val,
                "FILE_EXTENSION_COLOR" => result.file_extension_color = val,
                "FILE_TIME_FORMAT" => result.file_time_format = val,
                "FILE_TIME_TYPE" => result.file_time_type = val,
                "TREE_LAYER_LIMIT" => result.tree_layer_limit = val,
                "SHOW_FILE_METADATA" => result.show_file_metadata = val,
                "SHOW_DIR_METADATA" => result.show_dir_metadata = val,
                "PIPE" => result.pipe = val,
                "ELBOW" => result.elbow = val,
                "TEE" => result.tee = val,
                "PIPE_PREFIX" => result.pipe_prefix = val,
                "SPACE_PREFIX" => result.space_prefix = val,
                "DIR_NAME_STYLE" => result.dir_name_style = val,
                "FILE_NAME_STYLE" => result.file_name_style = val,
                "FILE_TIME_STYLE" => result.file_time_style = val,
                "FILE_SIZE_STYLE" => result.file_size_style = val,
                "FILE_OWNER_STYLE" => result.file_owner_style = val,
                "FILE_PERMS_STYLE" => result.file_perms_style = val,
                "FILE_EXTENSION_STYLE" => result.file_extension_style = val,
            }
        }

        return result
    }
}