use std::{env, fs};
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigInput {
    pub file_size_position: String,
    pub file_owner_position: String,
    pub file_perms_position: String,
    pub file_time_position: String,
    pub file_extension_position: String,
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
    pub tree_layer_limit: String,
    pub show_file_metadata: String,
    pub show_dir_metadata: String,
    pub pipe: String,
    pub elbow: String,
    pub tee: String,
    pub pipe_prefix: String,
    pub space_prefix: String,
    pub num_positions: String,
    pub dir_num_positions: String,
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
    pub spacing: String,
    pub show_short: String
}

#[derive(Debug)]
pub struct ConfigManager {
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

fn configure_variables() -> ConfigInput {
    let default_config : ConfigInput = ConfigInput {
        file_size_position: "1".to_string(),
        file_owner_position: "2".to_string(),
        file_perms_position: "3".to_string(),
        file_time_position: "4".to_string(),
        file_extension_position: "-1".to_string(),
        dir_name_color: "LIGHTRED".to_string(),
        file_name_color: "LIGHTRED".to_string(),
        file_time_color: "LIGHTCYAN".to_string(),
        file_size_color: "BLUE".to_string(),
        file_owner_color: "LIGHTMAGENTA".to_string(),
        file_perms_color: "BLUE".to_string(),
        file_extension_color: "YELLOW".to_string(),
        dir_name_style: "LIGHTCYAN".to_string(),
        file_name_style: "NORMAL".to_string(),
        file_time_style: "BOLD".to_string(),
        file_size_style: "BOLD".to_string(),
        file_owner_style: "NORMAL".to_string(),
        file_perms_style: "BOLD".to_string(),
        file_extension_style: "ITALIC".to_string(),
        file_time_format: "%m-%d-%Y::%H:%M:%S".to_string(),
        file_time_type: "CREATED".to_string(),
        tree_layer_limit: "3".to_string(),
        show_file_metadata: "TRUE".to_string(),
        show_dir_metadata: "TRUE".to_string(),
        pipe: "│".to_string(),
        elbow: "└──".to_string(),
        tee: "├──".to_string(),
        pipe_prefix: "│".to_string(),
        space_prefix: " ".to_string(),
        num_positions: "4".to_string(),
        dir_num_positions: "4".to_string(),
        dir_color: "BLUE".to_string(),
        symlink_color: "LIGHTMAGENTA".to_string(),
        path_color: "WHITE".to_string(),
        pipe_color: "YELLOW".to_string(),
        chard_color: "YELLOW".to_string(),
        blockd_color: "LIGHTGREEN".to_string(),
        socket_color: "LIGHTRED".to_string(),
        read_color: "LIGHTGREEN".to_string(),
        write_color: "LIGHTRED".to_string(),
        execute_color: "LIGHTGREEN".to_string(),
        dash_color: "LIGHTBLACK".to_string(),
        spacing: "0".to_string(),
        show_short: "false".to_string()
    };
    if let Some(proj_dirs) = ProjectDirs::from(
        "dev",
        "ctv",
        "ctv",
    ) {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(
            config_dir.join("config.toml"),
        );

        println!("f {}", config_dir.display());

        let config: ConfigInput = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => {
            let config_message = format!("Config file not created. Please visit https://github.com/angelina-tsuboi/ctv/blob/main/README.md to learn how to set up a config.toml file for CTV \n Create a config.toml file at {}", config_dir.display());
            println!("{}", config_message);
            default_config
            }
        };

        return config;
    }

    default_config
}

impl ConfigManager {
    pub fn init() -> Self {  
        let config_file : ConfigInput = configure_variables();  

        println!("cof is {:?}", config_file);

        let mut original : i32 = 5;
        if config_file.file_size_position.parse::<i32>().unwrap() == -1 {original -= 1};
        if config_file.file_owner_position.parse::<i32>().unwrap() == -1 {original -= 1};
        if config_file.file_perms_position.parse::<i32>().unwrap() == -1 {original -= 1};
        if config_file.file_time_position.parse::<i32>().unwrap() == -1 {original -= 1};
        if config_file.file_extension_position.parse::<i32>().unwrap() == -1 {original -= 1};

        let mut dir_num_pos : i32 = original;
        if config_file.file_extension_position.parse::<i32>().unwrap() != -1 {
            dir_num_pos -= 1;
        }

        let mut show_result = true;
        if config_file.show_short.to_uppercase() == "FALSE" {
            show_result = false;
        }

        Self {
            file_size_position: config_file.file_size_position.parse::<i32>().unwrap(),
            file_owner_position: config_file.file_owner_position.parse::<i32>().unwrap(),
            file_perms_position: config_file.file_perms_position.parse::<i32>().unwrap(),
            file_time_position: config_file.file_time_position.parse::<i32>().unwrap(),
            file_extension_position: config_file.file_extension_position.parse::<i32>().unwrap(),
            dir_name_color: config_file.dir_name_color,
            file_name_color: config_file.file_name_color,
            file_time_color: config_file.file_time_color,
            file_size_color: config_file.file_size_color,
            file_owner_color: config_file.file_owner_color,
            file_perms_color: config_file.file_perms_color,
            file_extension_color: config_file.file_extension_color,
            dir_name_style: config_file.dir_name_style,
            file_name_style: config_file.file_name_style,
            file_time_style: config_file.file_time_style,
            file_size_style: config_file.file_size_style,
            file_owner_style: config_file.file_owner_style,
            file_perms_style: config_file.file_perms_style,
            file_extension_style: config_file.file_extension_style,
            file_time_format: config_file.file_time_format,
            file_time_type: config_file.file_time_type,
            tree_layer_limit: config_file.tree_layer_limit.parse::<i32>().unwrap(),
            show_file_metadata: config_file.show_file_metadata,
            show_dir_metadata: config_file.show_dir_metadata,
            pipe: config_file.pipe,
            elbow: config_file.elbow,
            tee: config_file.tee,
            pipe_prefix: config_file.pipe_prefix,
            space_prefix: config_file.space_prefix,
            num_positions: original,
            dir_num_positions: dir_num_pos,
            dir_color: config_file.dir_color,
            symlink_color: config_file.symlink_color,
            path_color: config_file.path_color,
            pipe_color: config_file.pipe_color,
            chard_color: config_file.chard_color,            
            blockd_color: config_file.blockd_color,
            socket_color: config_file.socket_color,
            read_color: config_file.read_color,
            write_color: config_file.write_color,            
            execute_color: config_file.execute_color,
            dash_color: config_file.dash_color,
            spacing: config_file.spacing.parse::<i32>().unwrap(),
            show_short: show_result
        }
    }
}