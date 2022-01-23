use std::path::{PathBuf};
use crate::protocols::{TreeGenerator, ConfigManager};

#[derive(Clone)]
pub struct DirTree {
    tree_gen: TreeGenerator,
    tree: Vec<String>,
}

impl DirTree { 
    pub fn init(root_dir: PathBuf, env_manager: &ConfigManager) -> Self{
        let new_config : ConfigManager = env_manager.clone();
        Self {
            tree_gen: TreeGenerator::init(root_dir, new_config),
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