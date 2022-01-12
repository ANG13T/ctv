extern crate dotenv;
use dotenv::dotenv;
use std::{env};

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
    show_file_metadata: String,
    show_dir_metadata: String,
    pipe: String,
    elbow: String,
    tee: String,
    pipe_prefix: String,
    space_prefix: String,
}

impl EnvManager {
    pub fn init() -> Self {
        dotenv().ok();

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
        }
    }
}