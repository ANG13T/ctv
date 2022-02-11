use humansize::{file_size_opts as options, FileSize};
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

pub fn size(metadata: &Metadata) -> anyhow::Result<String> {
    metadata
        .size()
        .file_size(options::CONVENTIONAL)
        .map_err(|err| anyhow::anyhow!(err))
}
