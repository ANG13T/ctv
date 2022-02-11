use crate::config::Config;
use crate::protocols::file::File;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct DirTree<'a> {
    root: &'a Path,
    config: &'a Config,
    depth: usize,
    is_last: bool,
}

impl<'a> DirTree<'a> {
    pub fn new(root: &'a Path, config: &'a crate::config::Config) -> Self {
        Self {
            root,
            config,
            depth: 0,
            is_last: false,
        }
    }

    /// std::fmt::Display does not allow for any non-formatting errors to be propagated
    pub fn write(&self, formatter: &mut dyn Write) -> anyhow::Result<()> {
        self.write_header(formatter)?;
        self.write_body(formatter)
    }
    fn write_header(&self, formatter: &mut dyn Write) -> anyhow::Result<()> {
        // print (self.indentation - 1) pipes, then 1 tee if we have indentation at all
        for _ in 0..self.depth.saturating_sub(1) {
            write!(formatter, "{}", self.config.symbols.pipe)?;
        }
        if self.depth > 0 {
            write!(
                formatter,
                "{} ",
                if self.is_last {
                    &self.config.symbols.elbow
                } else {
                    &self.config.symbols.tee
                }
            )?;
        }
        writeln!(formatter, "{}", File::new(&self.root, &self.config)?)?;
        Ok(())
    }
    fn write_body(&self, formatter: &mut dyn Write) -> anyhow::Result<()> {
        use anyhow::Context;
        struct TreeContext(std::ffi::OsString);
        impl std::fmt::Display for TreeContext {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Failed to generate tree for entry {:?}", self.0)
            }
        }

        if self.root.symlink_metadata()?.file_type().is_dir() {
            let entries: Vec<std::fs::DirEntry> =
                fs::read_dir(self.root)?.collect::<std::io::Result<_>>()?;
            let num_entries = entries.len();
            for (idx, entry) in entries.into_iter().enumerate() {
                // not using Self because we need a shorter lifetime than our own, and Self forwards the lifetime parameter
                DirTree {
                    root: &entry.path(),
                    config: &self.config,
                    depth: self.depth + 1,
                    is_last: idx == num_entries - 1,
                }
                .write(formatter)
                .context(TreeContext(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
