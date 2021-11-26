extern crate dotenv;
use dotenv::dotenv;
use structopt::StructOpt;
use std::env;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;
use std::path::{PathBuf};


fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }
    let current_dir = env::current_dir()?;
    // readdirLoop(current_dir, 2, 2);
    let out_dir = env::var("PIPE").unwrap();
    println!("{0}", out_dir);
    Ok(())
}

// fn readdirLoop(dir: PathBuf, amount: i8, initialAmount: i8) -> Result<(), Box<dyn Error>>{
//     let mut count = 0;
//     for entry in fs::read_dir(dir)? {
//         let entry = entry?;
//         let path = entry.path();
//         let isLast = fs::read_dir(dir).unwrap().count() - 1 == count;

//         let metadata = fs::metadata(&path)?;
//         let last_modified = metadata.modified()?.elapsed()?.as_secs();

//         if metadata.is_file(){
//             let coolFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount, false, isLast);
//             print!("{:?}", coolFile);

//         }else if metadata.is_dir(){
//             if amount > 0 {
//                 let dirFile = protocols::File::new(entry.path(), input::Cli::from_args().created_time.to_string(), initialAmount - amount, true, isLast);
//                 print!("{:?}", dirFile);
//                 readdirLoop(entry.path(), amount - 1, initialAmount);
//             }
//         }
//         count += 1;
//     }
//     Ok(())
// }


#[cfg(test)]
mod test_suite;