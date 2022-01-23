use structopt::StructOpt;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{
    let check_config = protocols::ConfigManager::init();

    if !protocols::checkconfig::check_config(&check_config) {
        Err("ENV variables not declared properly")?
    }
    let modify_result = modify_env_with_flags();
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, &check_config);
    if !modify_result {
        dir_tree.gen();
    }
    Ok(())
}

fn modify_env_with_flags() -> bool {
    let set_env: &str = &input::Cli::from_args().set_env.clone();
    let layer: &str = &input::Cli::from_args().layer.clone();
    if set_env != "" && check_valid_set_env(set_env.to_string()){
        set_env_var(&set_env);
        return false;
    }

    if input::Cli::from_args().show_env {
        protocols::checkconfig::print_env();
        return true;
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

    return false;
}


fn make_concat_env(var_1: String, var_2: String) -> String {
    return format!("{}={}", var_1, var_2);
}

fn set_env_var(env_string: &str){
    let string_vec: Vec<&str> = env_string.split("=").collect();
    env::set_var(string_vec[0].to_uppercase(), string_vec[1].to_uppercase());
}


fn check_valid_set_env(env_input: String) -> bool{
    let used_positions: Vec<String> = protocols::checkconfig::get_used_positions();
    let string_vec: Vec<&str> = env_input.split("=").collect();
    if string_vec.len() != 2{
        println!("ERROR: invalid flag variable for --set-env");
        return false;
    }
    return protocols::checkconfig::check_env_var(string_vec[0], string_vec[1], &used_positions);
}



#[cfg(test)]
mod test_suite;