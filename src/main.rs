use structopt::StructOpt;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;

fn main() -> Result<(), Box<dyn Error>>{
    if !protocols::checkenv::check_env() {
        Err("ENV variables not declared properly")?
    }
    let env_manager = protocols::EnvManager::init();
    let updated_env_manager = modify_env_with_flags(env_manager);
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, updated_env_manager);
    dir_tree.gen();
    Ok(())
}

fn modify_env_with_flags(env_manager: protocols::EnvManager) -> protocols::EnvManager{
    if input::Cli::from_args().set_env != "" {

    }
    return env_manager;
}


// fn check_valid_set_env(env_input: String) -> bool{

// }


#[cfg(test)]
mod test_suite;