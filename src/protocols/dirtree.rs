use std::path::{PathBuf};
use crate::protocols::{TreeGenerator, EnvManager};

#[derive(Clone)]
pub struct DirTree {
    tree_gen: TreeGenerator,
    tree: Vec<String>,
}

impl DirTree { 
    pub fn init(root_dir: PathBuf, env_manager: EnvManager) -> Self{
        Self {
            tree_gen: TreeGenerator::init(root_dir, env_manager),
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