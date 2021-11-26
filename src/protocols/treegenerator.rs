use std::path::{PathBuf};

pub struct TreeGenerator {
    root_dir: PathBuf,
    tree: Vec<String>
}

impl TreeGenerator {
    fn init(&self, root_dir: PathBuf) {
        self.tree = Vec::new();
    }
    fn build_tree(&self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir);
        return self.tree;
    }
    fn tree_head(&self) {

    }
    fn tree_body(&self, directory: PathBuf, prefix: String) {
        prefix = "".to_string();
        
    }
    fn add_directory(&self, directory, index, entries_count, prefix, connector) {

    }
    fn add_file(&self, file, prefix, connector) {

    }
}