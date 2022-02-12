use humansize::{file_size_opts as options, FileSize};
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

pub fn get(metadata: &Metadata) -> u64 {
    metadata.size()
}

pub fn format(size: u64) -> String {
    size.file_size(options::CONVENTIONAL).unwrap()
}
