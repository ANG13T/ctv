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
   
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir);
    dir_tree.gen();
    Ok(())
}


#[cfg(test)]
mod test_suite;