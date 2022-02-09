use structopt::StructOpt;
mod input;
use std::error::Error;
mod decorators;
mod protocols;
mod services;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config_input = protocols::configmanager::configure_variables();

    let set_var: &str = &input::Cli::from_args().set_var.clone();

    if set_var != "" && check_valid_set_var(set_var.to_string(), config_input.clone()) {
        config_input = set_config_var(&set_var, config_input.clone());
    }

    let mut check_config = protocols::ConfigManager::init(config_input.clone());

    if !protocols::checkconfig::check_config(&config_input) {
        Err("Config variables not declared properly")?
    }

    if input::Cli::from_args().show_env {
        protocols::checkconfig::print_config(&check_config);
        return Ok(());
    }

    check_config = modify_config_with_flags(&check_config);

    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, &check_config);
    dir_tree.gen();
    Ok(())
}

fn modify_config_with_flags(
    config_input: &protocols::configmanager::ConfigManager,
) -> protocols::configmanager::ConfigManager {
    let layer: &str = &input::Cli::from_args().layer.clone();
    let mut new_config: protocols::ConfigManager = config_input.clone();
    let used_pos = vec![];

    if layer != "" && protocols::checkconfig::check_env_var("TREE_LAYER_LIMIT", layer, &used_pos) {
        new_config.tree_layer_limit = layer.parse::<i32>().unwrap();
    }

    if input::Cli::from_args().created_time {
        new_config.file_time_type = "CREATED".to_string();
    }

    if input::Cli::from_args().short {
        new_config.show_short = true;
    }

    return new_config;
}

// fn make_concat_env(var_1: String, var_2: String) -> String {
//     return format!("{}={}", var_1, var_2);
// }

fn set_config_var(
    env_string: &str,
    initial_config: protocols::configmanager::ConfigInput,
) -> protocols::configmanager::ConfigInput {
    let mut resultant = initial_config.clone();
    let (lower_var, ind_2) = match env_string.split_once('=') {
        Some((env_key, env_value)) => (env_key.to_lowercase(), env_value.to_uppercase()),
        None => {
            panic!("ERROR: invalid environment variable string");
        }
    };

    if lower_var == "file_size_position" {
        resultant.file_size_position = ind_2.clone()
    }
    if lower_var == "file_owner_position" {
        resultant.file_owner_position = ind_2.clone()
    }
    if lower_var == "file_perms_position" {
        resultant.file_perms_position = ind_2.clone()
    }
    if lower_var == "file_time_position" {
        resultant.file_time_position = ind_2.clone()
    }
    if lower_var == "file_extension_position" {
        resultant.file_extension_position = ind_2.clone()
    }
    if lower_var == "dir_name_color" {
        resultant.dir_name_color = ind_2.clone()
    }
    if lower_var == "file_name_color" {
        resultant.file_name_color = ind_2.clone()
    }
    if lower_var == "file_time_color" {
        resultant.file_time_color = ind_2.clone()
    }
    if lower_var == "file_size_color" {
        resultant.file_size_color = ind_2.clone()
    }
    if lower_var == "file_owner_color" {
        resultant.file_owner_color = ind_2.clone()
    }
    if lower_var == "file_extension_color" {
        resultant.file_extension_color = ind_2.clone()
    }
    if lower_var == "file_name_style" {
        resultant.file_name_style = ind_2.clone()
    }
    if lower_var == "file_time_style" {
        resultant.file_time_style = ind_2.clone()
    }
    if lower_var == "file_size_style" {
        resultant.file_size_style = ind_2.clone()
    }
    if lower_var == "file_owner_style" {
        resultant.file_owner_style = ind_2.clone()
    }
    if lower_var == "file_perms_style" {
        resultant.file_perms_style = ind_2.clone()
    }
    if lower_var == "file_extension_style" {
        resultant.file_extension_style = ind_2.clone()
    }
    if lower_var == "file_time_format" {
        resultant.file_time_format = ind_2.clone()
    }
    if lower_var == "file_time_type" {
        resultant.file_time_type = ind_2.clone()
    }
    if lower_var == "tree_layer_limit" {
        resultant.tree_layer_limit = ind_2.clone()
    }
    if lower_var == "show_file_metadata" {
        resultant.show_file_metadata = ind_2.clone()
    }
    if lower_var == "show_dir_metadata" {
        resultant.show_dir_metadata = ind_2.clone()
    }
    if lower_var == "elbow" {
        resultant.elbow = ind_2.clone()
    }
    if lower_var == "tee" {
        resultant.tee = ind_2.clone()
    }
    if lower_var == "pipe_prefix" {
        resultant.pipe_prefix = ind_2.clone()
    }
    if lower_var == "space_prefix" {
        resultant.space_prefix = ind_2.clone()
    }
    if lower_var == "dir_color" {
        resultant.dir_color = ind_2.clone()
    }
    if lower_var == "symlink_color" {
        resultant.symlink_color = ind_2.clone()
    }
    if lower_var == "path_color" {
        resultant.path_color = ind_2.clone()
    }
    if lower_var == "pipe_color" {
        resultant.pipe_color = ind_2.clone()
    }
    if lower_var == "chard_color" {
        resultant.chard_color = ind_2.clone()
    }
    if lower_var == "blockd_color" {
        resultant.blockd_color = ind_2.clone()
    }
    if lower_var == "socket_color" {
        resultant.socket_color = ind_2.clone()
    }
    if lower_var == "read_color" {
        resultant.read_color = ind_2.clone()
    }
    if lower_var == "write_color" {
        resultant.write_color = ind_2.clone()
    }
    if lower_var == "execute_color" {
        resultant.execute_color = ind_2.clone()
    }
    if lower_var == "dash_color" {
        resultant.dash_color = ind_2.clone()
    }
    if lower_var == "spacing" {
        resultant.spacing = ind_2.clone()
    }
    if lower_var == "show_short" {
        resultant.show_short = ind_2.clone()
    }

    resultant
}

fn check_valid_set_var(
    env_input: String,
    config_input: protocols::configmanager::ConfigInput,
) -> bool {
    let used_positions: Vec<String> = protocols::checkconfig::get_used_positions(&config_input);
    let (env_key, env_value) = match env_input.split_once('=') {
        Some(split) => split,
        None => {
            println!("ERROR: invalid flag variable for --set-var");
            return false;
        }
    };
    return protocols::checkconfig::check_env_var(env_key, env_value, &used_positions);
}

#[cfg(test)]
mod test_suite;
