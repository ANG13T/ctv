use std::path::{PathBuf};
use std::env;

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
        self.tree = Vec::new();

    }
    fn build_tree(&self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), "".to_string());
        return self.tree.clone();
    }
    fn tree_head(&self) {

    }
    fn tree_body(&self, directory: PathBuf, prefix: String) {
        

    }
    // fn add_directory(&self, directory, index, entries_count, prefix, connector) {

    // }
    // fn add_file(&self, file, prefix, connector) {

    // }
}