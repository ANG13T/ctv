extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};

pub fn check_env() -> bool {
    let all_var_names = ["PIPE".to_string(), "ELBOW".to_string(), "TEE".to_string(), "PIPE_PREFIX".to_string(), "SPACE_PREFIX".to_string(), "SHOW_FILE_METADATA".to_string(), "SHOW_DIR_METADATA".to_string()];
    let all_colors = ["BLACK".to_string(), "BLUE".to_string(), "CYAN".to_string(), "GREEN".to_string(), "LIGHTBLACK".to_string(), "LIGHTBLUE".to_string(), "LIGHTCYAN".to_string(), "LIGHTGREEN".to_string(), "LIGHTMAGENTA".to_string(), "LIGHTRED".to_string(), "LIGHTWHITE".to_string(), "LIGHTYELLOW".to_string(), "MAGENTA".to_string(), "RED".to_string(), "WHITE".to_string(), "YELLOW".to_string()];
    let all_styles = ["BOLD".to_string(), "UNDERLINE".to_string(), "DIMMED".to_string(), "ITALIC".to_string(), "BLINK".to_string(), "REVERSE".to_string(), "HIDDEN".to_string(), "STRICKEN".to_string()];
    dotenv().ok();
    for (key, val) in env::vars() {
        if key != "SPACE_PREFIX" && all_var_names.contains(&key) && val.len() == 0 {
            println!("ERROR: Invalid ENV variable with key {}. ENV variable must have a value", key);
            return false;
        }

        if is_color_path(&key) {
            if !all_colors.contains(&val.to_uppercase()){
                println!("ERROR: ENV variable with invalid color name. {} for variable {} is not a valid color!", val, key);
                return false;
            }
        }

        if is_style_path(&key) {
            if !all_styles.contains(&val.to_uppercase()){
                println!("ERROR: ENV variable with invalid style name. {} for variable {} is not a valid style!", val, key);
                return false;
            }
        }


    }
    true
}

fn is_color_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"COLOR");
}

fn is_style_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"STYLE");
}

fn is_position_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"POSITION");
}