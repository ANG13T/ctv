use std::path::{Path, PathBuf};
use crate::protocols::{TreeGenerator};

#[derive(Clone)]
pub struct DirTree {
    tree_gen: TreeGenerator,
    tree: Vec<String>,
}

impl DirTree { 
    pub fn init(root_dir: PathBuf) -> Self{
        Self {
            tree_gen: TreeGenerator::init(root_dir),
            tree: Vec::new()
        }
    }

    pub fn gen(&mut self) {
        self.tree = self.tree_gen.build_tree();
        for entry in &self.tree {
            println!("{}", entry);
        }
    }
}