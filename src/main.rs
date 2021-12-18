use std::{env};
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;


fn main() -> Result<(), Box<dyn Error>>{
    let current_dir = env::current_dir()?;
    let mut dir_tree = protocols::DirTree::init(current_dir);
    dir_tree.gen();
    Ok(())
}


#[cfg(test)]
mod test_suite;