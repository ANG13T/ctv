use structopt::StructOpt;
use std::{env, fs};
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;
use std::path::{PathBuf};

fn main() -> Result<(), Box<dyn Error>>{
    let current_dir = env::current_dir()?;
    readdirLoop(current_dir, 2, 2);
    Ok(())
}

fn readdirLoop(dir: PathBuf, amount: i8, initialAmount: i8) -> Result<(), Box<dyn Error>>{
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if metadata.is_file(){
            let coolFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount, false);
            print!("{:?}", coolFile);

        }else if metadata.is_dir(){
            if amount > 0 {
                let dirFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount, true);
                print!("{:?}", dirFile);
                readdirLoop(entry.path(), amount - 1, initialAmount);
            }
        }
    }
    Ok(())
}



#[cfg(test)]
mod test_suite;