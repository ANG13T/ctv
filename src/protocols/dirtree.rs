
#[derive(Clone)]
pub struct DirTree {
    tree_gen: TreeGenerator,
    tree: Vec<String>,
}

impl DirTree { 
    fn init(root_dir: PathBuf){
        self.tree_gen = TreeGenerator::new(root_dir);
    }

    fn gen(self){
        self.tree = self.tree_gen.build_tree();
        for entry in self.tree {
            println!(entry);
        }
    }
}