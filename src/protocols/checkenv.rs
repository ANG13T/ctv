extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};

pub fn check_env() -> bool {
    let all_var_names = ["PIPE".to_string(), "ELBOW".to_string(), "TEE".to_string(), "PIPE_PREFIX".to_string(), "SPACE_PREFIX".to_string(), "SHOW_FILE_METADATA".to_string(), "SHOW_DIR_METADATA".to_string()];
    dotenv().ok();
    for (key, val) in env::vars() {
        if key != "SPACE_PREFIX" && all_var_names.contains(&key) && val.len() == 0 {
            println!("Invalid ENV variable with key {}. ENV variable must have a value", key);
            return false;
        }
    }
    true
}