extern crate dotenv;
use dotenv::dotenv;
use std::{env};

pub fn check_env() -> bool {
    let all_var_names = ["PIPE".to_string(), "ELBOW".to_string(), "TEE".to_string(), "PIPE_PREFIX".to_string(), "SPACE_PREFIX".to_string(), "SHOW_FILE_METADATA".to_string(), "SHOW_DIR_METADATA".to_string()];
    let all_colors = ["BLACK".to_string(), "BLUE".to_string(), "CYAN".to_string(), "GREEN".to_string(), "LIGHTBLACK".to_string(), "LIGHTBLUE".to_string(), "LIGHTCYAN".to_string(), "LIGHTGREEN".to_string(), "LIGHTMAGENTA".to_string(), "LIGHTRED".to_string(), "LIGHTWHITE".to_string(), "LIGHTYELLOW".to_string(), "MAGENTA".to_string(), "RED".to_string(), "WHITE".to_string(), "YELLOW".to_string()];
    let all_styles = ["BOLD".to_string(), "UNDERLINE".to_string(), "DIMMED".to_string(), "ITALIC".to_string(), "BLINK".to_string(), "REVERSE".to_string(), "HIDDEN".to_string(), "STRICKEN".to_string()];
    let mut used_positions = vec![];
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

            if is_valid_rgb(&val.to_uppercase(), &key) {
                println!("ERROR: ENV variable with invalid RGB value for color. {} for variable {} is not a valid RGB value!", val, key);
                return false;
            }
        }

        if is_style_path(&key) {
            if !all_styles.contains(&val.to_uppercase()){
                println!("ERROR: ENV variable with invalid style name. {} for variable {} is not a valid style!", val, key);
                return false;
            }
        }

        if is_position_path(&key) {
            let key_int: i32 = val.parse::<i32>().ok().expect("INVALID integer in env variable!");
            if key_int <= 0 || key_int > 3 {
                println!("ERROR: ENV variable with invalid position range. Position {} for variable {} is out of range! Position should be 1, 2, or 3!", val, key);
                return false;
            }
            if used_positions.contains(&key) {
                println!("ERROR: ENV variable with invalid position. Position {} for variable {} has already been used! Please consider giving it a different position", val, key);
                return false;
            }
            used_positions.push(val);
        }

    }
    true
}

fn is_color_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"COLOR");
}

fn is_valid_rgb(color: &str, path: &str) -> bool {
    let uppercased_no_space: String = color.to_uppercase().replace(" ", "");
    if &uppercased_no_space[..4] != "RGB(" || &uppercased_no_space[&uppercased_no_space.len()-1..] != ")" {return false};
    let substring: &str = &uppercased_no_space[4..uppercased_no_space.len()];
    let split_values: Vec<&str> = substring.split(",").collect();
    if split_values.len() != 3 { return false };
    for value in split_values {
        let rgb_number : i32 = value.parse::<i32>().ok().expect("Invalid RGB number value in env");
        if rgb_number < 0 || rgb_number > 255 {
            println!("INVALID RGB number value of {} for {}", rgb_number, path);
            return false;
        }
    }
    return true;
}

fn is_style_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"STYLE");
}

fn is_position_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"POSITION");
}