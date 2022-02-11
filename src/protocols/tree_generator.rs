use crate::config::Config;
use crate::protocols::file::File;
use std::fs;
use std::path::Path;

pub struct TreeGenerator<'a> {
    root_dir: &'a Path,
    tree: Vec<String>,
    config: Config,
    spacing: usize,
}

impl<'a> TreeGenerator<'a> {
    pub fn init(root_dir: &'a Path, config: crate::config::Config) -> Self {
        Self {
            tree: Vec::new(),
            root_dir,
            config,
            spacing: 0,
        }
    }
    pub fn build_tree(mut self) -> anyhow::Result<Vec<String>> {
        self.tree_head()?;
        self.tree_body(self.root_dir, "", self.config.max_depth)?;
        Ok(self.tree)
    }

    fn sort_dir_first(&self, directory: &Path) -> anyhow::Result<Vec<fs::DirEntry>> {
        let mut ret = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
        ret.sort_by_cached_key(|entry| !entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false));
        Ok(ret)
    }

    fn tree_head(&mut self) -> anyhow::Result<()> {
        let dir_file = File::new(&self.root_dir, &self.config)?;
        self.tree.push(dir_file.to_string()); // prints out head dir
        Ok(())
    }

    fn tree_body(&mut self, directory: &Path, prefix: &str, limit: usize) -> anyhow::Result<()> {
        eprintln!("tree_body {}", directory.display());
        let entries = self.sort_dir_first(directory)?;
        let entries_count = entries.len();

        for (index, entry) in entries.iter().enumerate() {
            eprintln!("entry: {} {:?}", index, entry);
            let metadata = fs::metadata(entry.path())?;

            let connector = if index == entries_count - 1
                && (!metadata.is_dir() || entry.path().components().count() == 0)
            {
                self.config.symbols.elbow.clone()
            } else {
                self.config.symbols.tee.clone()
            };

            if metadata.is_dir() {
                self.add_directory(
                    &entry.path(),
                    index,
                    entries_count,
                    &prefix,
                    &connector,
                    limit - 1,
                )?;
            } else {
                self.add_file(&entry.path(), &prefix, &connector)?;
            }
        }
        Ok(())
    }

    fn add_spacing(&mut self, prefix: &str) {
        for _ in 0..self.spacing {
            self.tree
                .push(format!("{}{}", prefix, self.config.symbols.pipe))
        }
    }

    fn add_directory(
        &mut self,
        directory: &Path,
        index: usize,
        entries_count: usize,
        prefix: &str,
        connector: &str,
        limit: usize,
    ) -> anyhow::Result<()> {
        self.add_spacing(&prefix);
        let new_file = File::new(directory, &self.config)?;
        self.tree
            .push(format!("{}{} {}", prefix, connector, new_file));
        let prefix = if index != entries_count - 1 {
            [prefix, &self.config.symbols.pipe].join("")
        } else {
            [prefix, " "].join("")
        };
        if limit > 0 {
            self.tree_body(directory, &prefix, limit)?;
        }
        Ok(())
    }

    fn add_file(&mut self, file: &Path, prefix: &str, connector: &str) -> anyhow::Result<()> {
        self.add_spacing(prefix);
        let new_file = File::new(file, &self.config)?;
        self.tree
            .push(format!("{}{} {}", prefix, connector, new_file));
        Ok(())
    }
}
