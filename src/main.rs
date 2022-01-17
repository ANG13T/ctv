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
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, env_manager);
    dir_tree.gen();
    Ok(())
}

fn modify_env_with_flags(){
    
}


#[cfg(test)]
mod test_suite;