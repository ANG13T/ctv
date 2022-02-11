use crate::config::Config;
use crate::protocols::TreeGenerator;
use std::path::Path;

pub struct DirTree<'a> {
    tree_gen: TreeGenerator<'a>,
}

impl<'a> DirTree<'a> {
    pub fn init(root_dir: &'a Path, config: Config) -> Self {
        Self {
            tree_gen: TreeGenerator::init(root_dir, config),
        }
    }

    pub fn gen(self) -> anyhow::Result<()> {
        let tree = self.tree_gen.build_tree()?;
        for entry in tree.iter() {
            println!("{}", entry);
        }
        Ok(())
    }
}
