pub trait TreeGenerator {
    fn init(&self, root_dir);
    fn build_tree(&self);
    fn tree_head(&self);
    fn tree_body(&self);
    fn add_directory(&self, directory, index, entries_count, prefix, connector);
    fn add_file(&self, file, prefix, connector);
}

impl TreeGenerator {
    
}