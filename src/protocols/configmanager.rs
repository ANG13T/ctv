extern crate dotenv;
use dotenv::dotenv;
use std::{env};
use load_dotenv::load_dotenv;

#[derive(Debug)]
pub struct EnvManager {
    pub file_size_position: i32,
    pub file_owner_position: i32,
    pub file_perms_position: i32,
    pub file_time_position: i32,
    pub file_extension_position: i32,
    pub dir_name_color: String,
    pub file_name_color: String,
    pub file_time_color: String,
    pub file_size_color: String,
    pub file_owner_color: String,
    pub file_perms_color: String,
    pub file_extension_color: String,
    pub dir_name_style: String,
    pub file_name_style: String,
    pub file_time_style: String,
    pub file_size_style: String,
    pub file_owner_style: String,
    pub file_perms_style: String,
    pub file_extension_style: String,
    pub file_time_format: String,
    pub file_time_type: String,
    pub tree_layer_limit: i32,
    pub show_file_metadata: String,
    pub show_dir_metadata: String,
    pub pipe: String,
    pub elbow: String,
    pub tee: String,
    pub pipe_prefix: String,
    pub space_prefix: String,
    pub num_positions: i32,
    pub dir_num_positions: i32,
    pub dir_color: String,
    pub symlink_color: String,
    pub path_color: String,
    pub pipe_color: String,
    pub chard_color: String,
    pub blockd_color: String,
    pub socket_color: String,
    pub read_color: String,
    pub write_color: String,
    pub execute_color: String,
    pub dash_color: String,
    pub spacing: i32,
    pub show_short: bool
}


impl EnvManager {
    pub fn init() -> Self {
        dotenv().ok();
        load_dotenv!();

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

        let mut show_result = true;
        if env::var("SHOW_SHORT").unwrap() == "FALSE" {
            show_result = false;
        }

        Self {
            file_size_position: env::var("FILE_SIZE_POSITION").unwrap().parse::<i32>().unwrap(),
            file_owner_position: env::var("FILE_OWNER_POSITION").unwrap().parse::<i32>().unwrap(),
            file_perms_position: env::var("FILE_PERMS_POSITION").unwrap().parse::<i32>().unwrap(),
            file_time_position: env::var("FILE_TIME_POSITION").unwrap().parse::<i32>().unwrap(),
            file_extension_position: env::var("FILE_EXTENSION_POSITION").unwrap().parse::<i32>().unwrap(),
            dir_name_color: env::var("DIR_NAME_COLOR").unwrap(),
            file_name_color: env::var("FILE_NAME_COLOR").unwrap(),
            file_time_color: env::var("FILE_TIME_COLOR").unwrap(),
            file_size_color: env::var("FILE_SIZE_COLOR").unwrap(),
            file_owner_color: env::var("FILE_OWNER_COLOR").unwrap(),
            file_perms_color: env::var("FILE_PERMS_COLOR").unwrap(),
            file_extension_color: env::var("FILE_EXTENSION_COLOR").unwrap(),
            dir_name_style: env::var("DIR_NAME_STYLE").unwrap(),
            file_name_style: env::var("FILE_NAME_STYLE").unwrap(),
            file_time_style: env::var("FILE_TIME_STYLE").unwrap(),
            file_size_style: env::var("FILE_SIZE_STYLE").unwrap(),
            file_owner_style: env::var("FILE_OWNER_STYLE").unwrap(),
            file_perms_style: env::var("FILE_PERMS_STYLE").unwrap(),
            file_extension_style: env::var("FILE_EXTENSION_STYLE").unwrap(),
            file_time_format: env::var("FILE_TIME_FORMAT").unwrap(),
            file_time_type: env::var("FILE_TIME_TYPE").unwrap(),
            tree_layer_limit: env::var("TREE_LAYER_LIMIT").unwrap().parse::<i32>().unwrap(),
            show_file_metadata: env::var("SHOW_FILE_METADATA").unwrap(),
            show_dir_metadata: env::var("SHOW_DIR_METADATA").unwrap(),
            pipe: env::var("PIPE").unwrap(),
            elbow: env::var("ELBOW").unwrap(),
            tee: env::var("TEE").unwrap(),
            pipe_prefix: env::var("PIPE_PREFIX").unwrap(),
            space_prefix: env::var("SPACE_PREFIX").unwrap(),
            num_positions: original,
            dir_num_positions: dir_num_pos,
            dir_color: env::var("DIR_COLOR").unwrap(),
            symlink_color: env::var("SYMLINK_COLOR").unwrap(),
            path_color: env::var("PATH_COLOR").unwrap(),
            pipe_color: env::var("PIPE_COLOR").unwrap(),
            chard_color: env::var("CHARD_COLOR").unwrap(),            
            blockd_color: env::var("BLOCKD_COLOR").unwrap(),
            socket_color: env::var("SOCKET_COLOR").unwrap(),
            read_color: env::var("READ_COLOR").unwrap(),
            write_color: env::var("WRITE_COLOR").unwrap(),            
            execute_color: env::var("EXECUTE_COLOR").unwrap(),
            dash_color: env::var("DASH_COLOR").unwrap(),
            spacing: env::var("SPACING").unwrap().parse::<i32>().unwrap(),
            show_short: show_result
        }
    }
}