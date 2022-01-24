use structopt::StructOpt;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{
    let config_input = protocols::configmanager::configure_variables();  
    let mut check_config = protocols::ConfigManager::init(config_input);

    if !protocols::checkconfig::check_config(&check_config, &config_input) {
        Err("ENV variables not declared properly")?
    }
    check_config = modify_env_with_flags(&check_config);
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, &check_config);
    dir_tree.gen();
    Ok(())
}

fn modify_env_with_flags(config_input: &protocols::ConfigManager) -> ConfigManager {
    let set_env: &str = &input::Cli::from_args().set_env.clone();
    let layer: &str = &input::Cli::from_args().layer.clone();
    let mut new_config : ConfigManager = config_input.clone();

    if input::Cli::from_args().show_env {
        protocols::checkconfig::print_config(config_input);
        return new_config;
    }

    if layer != "" && check_valid_set_env(make_concat_env("TREE_LAYER_LIMIT".to_string(), layer.to_string())){
        set_env_var(&make_concat_env("TREE_LAYER_LIMIT".to_string(), layer.to_string()));
    }

    if input::Cli::from_args().created_time {
        set_env_var(&make_concat_env("FILE_TIME_TYPE".to_string(), "CREATED".to_string()));
    }

    if input::Cli::from_args().short {
        set_env_var(&make_concat_env("SHOW_SHORT".to_string(), "TRUE".to_string()));
    }

    return new_config;
}


#[cfg(test)]
mod test_suite;