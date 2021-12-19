extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};

pub fn check_env() -> bool {
    dotenv().ok();
    false
}