use std::time::{SystemTime};
use chrono::prelude::*;
use filetime::FileTime;
use std::{fs, env};
use std::path::PathBuf;

pub fn time_modified(file: PathBuf) -> String{
    let meta = fs::metadata(file).unwrap();
    let mtime = FileTime::from_last_modification_time(&meta);
    println!("{}", mtime);
    return "".to_string()
}

pub fn time_created(file: PathBuf) -> String{
    let meta = fs::metadata(file).unwrap();
    let mtime = FileTime::from_last_access_time(&meta);
    println!("{}", mtime);
    return "".to_string()
}