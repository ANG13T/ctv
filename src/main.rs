use structopt::StructOpt;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;

fn main() -> Result<(), Box<dyn Error>>{
    let config_input = protocols::configmanager::configure_variables();  
    let mut check_config = protocols::ConfigManager::init(config_input.clone());

    if !protocols::checkconfig::check_config(&check_config, &config_input) {
        Err("ENV variables not declared properly")?
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

fn modify_config_with_flags(config_input: &protocols::ConfigManager) -> protocols::ConfigManager {
    let set_var: &str = &input::Cli::from_args().set_var.clone();
    let layer: &str = &input::Cli::from_args().layer.clone();
    let mut new_config : protocols::ConfigManager = config_input.clone();
    let used_pos = vec![];

    if layer != "" && protocols::checkconfig::check_env_var("TREE_LAYER_LIMIT", layer, &used_pos){
        new_config.tree_layer_limit = layer.parse::<i32>().unwrap();
    }

    if set_var != "" && check_valid_set_var(set_var.to_string()){
        set_config_var(&set_var);
        return false;
    }

    if input::Cli::from_args().created_time {
        new_config.file_time_type = "CREATED".to_string();
    }

    if input::Cli::from_args().short {
        new_config.show_short = true;
    }

    return new_config;
}


fn make_concat_env(var_1: String, var_2: String) -> String {
    return format!("{}={}", var_1, var_2);
}

fn set_config_var(env_string: &str){
    let string_vec: Vec<&str> = env_string.split("=").collect();
    env::set_var(string_vec[0].to_uppercase(), string_vec[1].to_uppercase());
}

fn check_valid_set_var(env_input: String) -> bool{
    let used_positions: Vec<String> = protocols::checkconfig::get_used_positions();
    let string_vec: Vec<&str> = env_input.split("=").collect();
    if string_vec.len() != 2{
        println!("ERROR: invalid flag variable for --set-var");
        return false;
    }
    return protocols::checkconfig::check_env_var(string_vec[0], string_vec[1], &used_positions);
}

#[cfg(test)]
mod test_suite;