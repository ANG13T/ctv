use crate::decorators;
use crate::protocols::colormanager;
use crate::protocols::configmanager::{ConfigInput, ConfigManager};

struct ConfigView {
    property: String,
    value: String,
}

impl ConfigView {
    pub fn new(prop: &str, val: &str) -> Self {
        ConfigView {
            property: prop.to_string().to_uppercase(),
            value: val.to_string().to_uppercase(),
        }
    }
}

pub fn check_config(config_input: &ConfigInput) -> bool {
    let mut used_positions = vec![];
    let config_vars = to_config_view_array(config_input);
    for config_var in config_vars {
        if !check_env_var(&config_var.property, &config_var.value, &used_positions) {
            return false;
        }
        if is_position_path(&config_var.property) && config_var.value != "-1" {
            used_positions.push(config_var.value);
        }
    }
    true
}

fn to_config_view_array(config_input: &ConfigInput) -> Vec<ConfigView> {
    let config_array = vec![
        ConfigView::new("file_size_position", &config_input.file_size_position),
        ConfigView::new("file_owner_position", &config_input.file_owner_position),
        ConfigView::new("file_perms_position", &config_input.file_perms_position),
        ConfigView::new("file_time_position:", &config_input.file_time_position),
        ConfigView::new(
            "file_extension_position",
            &config_input.file_extension_position,
        ),
        ConfigView::new("dir_name_color", &config_input.dir_name_color),
        ConfigView::new("file_name_color", &config_input.file_name_color),
        ConfigView::new("file_time_color", &config_input.file_time_color),
        ConfigView::new("file_size_color", &config_input.file_size_color),
        ConfigView::new("file_owner_color", &config_input.file_owner_color),
        ConfigView::new("file_extension_color", &config_input.file_extension_color),
        ConfigView::new("file_name_style", &config_input.file_name_style),
        ConfigView::new("file_time_style", &config_input.file_time_style),
        ConfigView::new("file_size_style", &config_input.file_size_style),
        ConfigView::new("file_owner_style", &config_input.file_owner_style),
        ConfigView::new("file_perms_style", &config_input.file_perms_style),
        ConfigView::new("file_extension_style", &config_input.file_extension_style),
        ConfigView::new("file_time_format", &config_input.file_time_format),
        ConfigView::new("file_time_type", &config_input.file_time_type),
        ConfigView::new("tree_layer_limit", &config_input.tree_layer_limit),
        ConfigView::new("show_file_metadata", &config_input.show_file_metadata),
        ConfigView::new("show_dir_metadata", &config_input.show_dir_metadata),
        ConfigView::new("elbow", &config_input.elbow),
        ConfigView::new("tee", &config_input.tee),
        ConfigView::new("pipe_prefix", &config_input.pipe_prefix),
        ConfigView::new("space_prefix", &config_input.space_prefix),
        ConfigView::new("dir_color", &config_input.dir_color),
        ConfigView::new("symlink_color", &config_input.symlink_color),
        ConfigView::new("path_color", &config_input.path_color),
        ConfigView::new("pipe_color", &config_input.pipe_color),
        ConfigView::new("chard_color", &config_input.chard_color),
        ConfigView::new("blockd_color", &config_input.blockd_color),
        ConfigView::new("socket_color", &config_input.socket_color),
        ConfigView::new("read_color", &config_input.read_color),
        ConfigView::new("write_color", &config_input.write_color),
        ConfigView::new("execute_color", &config_input.execute_color),
        ConfigView::new("dash_color", &config_input.dash_color),
        ConfigView::new("spacing", &config_input.spacing),
        ConfigView::new("show_short", &config_input.show_short),
    ];
    config_array
}

pub fn print_config(config_input: &ConfigManager) {
    println!(
        "{}",
        colormanager::colorize_string("GREEN", format!("{:#?}", config_input))
    );
    println!("{}", colormanager::colorize_string("WHITE", "".to_string()));
}

pub fn get_used_positions(config_input: &ConfigInput) -> Vec<String> {
    let mut used_positions = vec![];
    let config_vars = to_config_view_array(config_input);
    for config_var in config_vars {
        if is_position_path(&config_var.property) && config_var.value != "-1" {
            used_positions.push(config_var.value);
        }
    }
    used_positions
}

pub fn check_env_var(key: &str, val: &str, used_positions: &[String]) -> bool {
    let all_var_names = [
        "PIPE".to_string(),
        "ELBOW".to_string(),
        "TEE".to_string(),
        "PIPE_PREFIX".to_string(),
        "SPACE_PREFIX".to_string(),
        "SHOW_FILE_METADATA".to_string(),
        "SHOW_DIR_METADATA".to_string(),
    ];
    let all_colors = [
        "BLACK".to_string(),
        "BLUE".to_string(),
        "CYAN".to_string(),
        "GREEN".to_string(),
        "LIGHTBLACK".to_string(),
        "LIGHTBLUE".to_string(),
        "LIGHTCYAN".to_string(),
        "LIGHTGREEN".to_string(),
        "LIGHTMAGENTA".to_string(),
        "LIGHTRED".to_string(),
        "LIGHTWHITE".to_string(),
        "LIGHTYELLOW".to_string(),
        "MAGENTA".to_string(),
        "RED".to_string(),
        "WHITE".to_string(),
        "YELLOW".to_string(),
    ];
    let all_styles = [
        "BOLD".to_string(),
        "UNDERLINE".to_string(),
        "DIMMED".to_string(),
        "ITALIC".to_string(),
        "BLINK".to_string(),
        "REVERSE".to_string(),
        "HIDDEN".to_string(),
        "STRICKEN".to_string(),
        "NORMAL".to_string(),
    ];
    let all_time_formats = [
        "CREATED".to_string(),
        "MODIFIED".to_string(),
        "ACCESSED".to_string(),
    ];
    let file_detail_num = 5;

    if key != "SPACE_PREFIX" && all_var_names.contains(&key.to_string()) && val.is_empty() {
        print_error(format!(
            "ERROR: Invalid config variable with key {}. config variable must have a value",
            key
        ));
        return false;
    }

    if is_color_path(key) {
        if !all_colors.contains(&val.to_uppercase()) {
            print_error(format!("ERROR: config variable with invalid color name. {} for variable {} is not a valid color!", val, key));
            return false;
        }

        if !all_colors.contains(&val.to_uppercase()) && !is_valid_rgb(&val.to_uppercase(), key) {
            print_error(format!("ERROR: config variable with invalid RGB value for color. {} for variable {} is not a valid RGB value!", val, key));
            return false;
        }
    }

    if is_style_path(key) {
        if !all_styles.contains(&val.to_uppercase()) {
            print_error(format!("ERROR: config variable with invalid style name. {} for variable {} is not a valid style!", val, key));
            return false;
        }
    }

    if is_metadata_path(key) {
        if &val.to_uppercase() != "TRUE" && &val.to_uppercase() != "FALSE" {
            print_error(format!("ERROR: config variable with invalid metadata name. {} for variable {} is not a valid variable! It must be either TRUE or FALSE", val, key));
            return false;
        }
    }

    if is_limit_path(key) {
        let key_int: i32 = val
            .parse::<i32>()
            .ok()
            .expect("INVALID integer for TREE_LAYER_LIMIT in env variable!");
        if key_int <= 0 {
            print_error(format!("ERROR: config variable with invalid tree layer limit. {} for variable {} is not a valid variable! It must be greater than 0", val, key));
            return false;
        }

        if key_int > 7 {
            print_error(format!("ERROR: config variable with invalid tree layer limit. {} for variable {} is not a valid variable! It must be less than 8", val, key));
            return false;
        }
    }

    if key == "FILE_TIME_TYPE" {
        if !all_time_formats.contains(&val.to_uppercase()) {
            print_error(format!("ERROR: config variable with invalid time type. {} for variable {} is not a valid time type! Valid time types are CREATED or MODIFIED", val, key));
            return false;
        }
    }

    if key == "SHOW_SHORT" {
        if val.to_uppercase() != "TRUE" && val.to_uppercase() != "FALSE" {
            print_error(format!("ERROR: config variable with invalid show short type. {} for variable {} is not a valid show short type! Valid types are TRUE or FALSE", val, key));
            return false;
        }
    }

    if key == "SPACING" {
        let key_int: i32 = val
            .parse::<i32>()
            .ok()
            .expect("INVALID integer for TREE_LAYER_LIMIT in env variable!");
        if key_int < 0 {
            print_error(format!("ERROR: config variable with invalid spacing amount. {} for variable {} is not a valid spacing! Spacing must be greater than or equal to 0", val, key));
            return false;
        }

        if key_int > 7 {
            print_error(format!("ERROR: config variable with invalid spacing amount. {} for variable {} is not a valid spacing! Spacing must be less than 7", val, key));
            return false;
        }
    }

    if is_position_path(key) {
        let key_int: i32 = val
            .parse::<i32>()
            .ok()
            .expect("INVALID integer in env variable!");
        if (key_int <= 0 && key_int != -1) || key_int > file_detail_num {
            print_error(format!("ERROR: config variable with invalid position range. Position {} for variable {} is out of range! Position should be -1, 1, 2, 3, 4, or 5!", val, key));
            return false;
        }
        if used_positions.contains(&key.to_string()) {
            print_error(format!("ERROR: config variable with invalid position. Position {} for variable {} has already been used! Please consider giving it a different position", val, key));
            return false;
        }
    }
    true
}

fn is_color_path(path: &str) -> bool {
    path.split('_').any(|segment| segment == "COLOR")
}

fn is_metadata_path(path: &str) -> bool {
    path.split('_').any(|segment| segment == "METADATA")
}

fn is_valid_rgb(color: &str, path: &str) -> bool {
    let uppercased_no_space: String = color.to_uppercase().replace(" ", "");
    if &uppercased_no_space[..4] != "RGB("
        || &uppercased_no_space[&uppercased_no_space.len() - 1..] != ")"
    {
        return false;
    };
    let substring: &str = &uppercased_no_space[4..uppercased_no_space.len()];
    let split_values: Vec<&str> = substring.split(',').collect();
    if split_values.len() != 3 {
        return false;
    };
    split_values
        .iter()
        .all(|segment| segment.parse::<u8>().is_ok())
}

fn is_style_path(path: &str) -> bool {
    path.split('_').any(|segment| segment == "STYLE")
}

fn print_error(error_message: String) {
    println!(
        "{}",
        decorators::bold(&colormanager::colorize_string("RED", error_message))
    );
}

fn is_position_path(path: &str) -> bool {
    path.split('_').any(|segment| segment == "POSITION")
}

fn is_limit_path(path: &str) -> bool {
    path.split('_').any(|segment| segment == "LIMIT")
}
