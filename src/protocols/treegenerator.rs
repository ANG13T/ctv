extern crate dotenv;
use dotenv::dotenv;
use std::env;
use structopt::StructOpt;
use std::path::{PathBuf};
use crate::protocols;
use crate::input;

pub struct TreeGenerator {
    root_dir: PathBuf,
    tree: Vec<String>,
    PIPE: String,
    ELBOW: String, 
    TEE: String,
    PIPE_PREFIX: String,
    SPACE_PREFIX: String
}

impl TreeGenerator {
    fn init(&mut self, root_dir: PathBuf) {
        dotenv().ok();
        self.tree = Vec::new();
        self.PIPE = env::var("PIPE").unwrap();
        self.ELBOW = env::var("ELBOW").unwrap();
        self.TEE = env::var("TEE").unwrap();
        self.PIPE_PREFIX = env::var("PIPE_PREFIX").unwrap();
        self.SPACE_PREFIX = env::var("SPACE_PREFIX").unwrap();
    }
    fn build_tree(&self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), "".to_string());
        return self.tree.clone();
    }
    fn tree_head(&self) {
        let dirFile = protocols::File::new(self.root_dir, input::Cli::from_args().created_time.to_string());
        self.tree.append(self.root_dir.);
        self.tree.append(self.PIPE);
    }
    fn tree_body(&self, directory: PathBuf, prefix: String) {
        

    }
    // fn add_directory(&self, directory, index, entries_count, prefix, connector) {

    // }
    // fn add_file(&self, file, prefix, connector) {

    // }
}