use crate::config::Config;
use crate::protocols::file::File;
use std::borrow::Cow;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct DirTree<'a> {
    root: File<'a>,
    config: &'a Config,
    depth: usize,
    is_last: bool,
}

impl<'a> DirTree<'a> {
    pub fn new(root: &'a Path, config: &'a crate::config::Config) -> anyhow::Result<Self> {
        Ok(Self {
            root: File::new(Cow::Borrowed(root), config)?,
            config,
            depth: 0,
            is_last: false,
        })
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
        writeln!(formatter, "{}", self.root)?;
        Ok(())
    }
    fn sort_entries(&self, entries: &mut Vec<File<'_>>) {
        use crate::config::SortType;
        let methods = self.config.sorting.as_slice();
        entries.sort_unstable_by(|a, b| {
            for method in methods {
                let cmp = match method.ty {
                    SortType::Name => a.path.file_name().cmp(&b.path.file_name()),
                    SortType::Size => a.size.cmp(&b.size),
                    SortType::Time => a.time.cmp(&b.time),
                    SortType::Type => a.file_type.cmp(&b.file_type),
                };
                if !cmp.is_eq() {
                    return if method.descending {
                        cmp.reverse()
                    } else {
                        cmp
                    };
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    fn write_body(&self, formatter: &mut dyn Write) -> anyhow::Result<()> {
        use anyhow::Context;
        struct TreeContext(String);
        impl std::fmt::Display for TreeContext {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Failed to generate tree for entry {:?}", self.0)
            }
        }

        if matches!(
            self.root.file_type,
            crate::protocols::PathType::Directory { .. }
        ) && self.depth < self.config.max_depth
        {
            let mut entries: Vec<File> = fs::read_dir(&self.root.path)?
                .map(|entry| File::new(Cow::Owned(entry?.path()), &self.config))
                .collect::<anyhow::Result<_>>()?;
            self.sort_entries(&mut entries);
            let num_entries = entries.len();
            for (idx, entry) in entries.into_iter().enumerate() {
                let name = entry.file_name().into_owned();
                // not using Self because we need a shorter lifetime than our own, and Self forwards the lifetime parameter
                DirTree {
                    root: entry,
                    config: &self.config,
                    depth: self.depth + 1,
                    is_last: idx == num_entries - 1,
                }
                .write(formatter)
                .context(TreeContext(name))?;
            }
        }
        Ok(())
    }
}
