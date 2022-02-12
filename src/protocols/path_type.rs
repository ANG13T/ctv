use crate::config::TypeColors;
use std::fs::Metadata;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathType {
    BlockDevice,
    CharDevice,
    Directory { num_entries: usize },
    File,
    Pipe,
    Socket,
    Symlink { target: PathBuf },
    Unknown,
}

impl PathType {
    pub fn from_path(file: &Path, metadata: Option<Metadata>) -> anyhow::Result<Self> {
        let metadata = match metadata {
            Some(metadata) => metadata,
            None => file.symlink_metadata()?,
        };
        let file_type = metadata.file_type();
        Ok(if file_type.is_symlink() {
            Self::Symlink {
                target: std::fs::read_link(file)?,
            }
        } else if file_type.is_fifo() {
            Self::Pipe
        } else if file_type.is_char_device() {
            Self::CharDevice
        } else if file_type.is_block_device() {
            Self::BlockDevice
        } else if file_type.is_socket() {
            Self::Socket
        } else if file_type.is_file() {
            Self::File
        } else if file_type.is_dir() {
            Self::Directory {
                num_entries: std::fs::read_dir(file)?.count(),
            }
        } else {
            Self::Unknown
        })
    }
    pub fn letter(&self) -> &'static str {
        match self {
            Self::BlockDevice => "b",
            Self::CharDevice => "c",
            Self::Directory { .. } => "d",
            Self::File => ".",
            Self::Pipe => "|",
            Self::Socket => "=",
            Self::Symlink { .. } => "l",
            Self::Unknown => "?",
        }
    }
    pub fn color(&self, colors: &TypeColors) -> crate::config::Color {
        match self {
            Self::BlockDevice => colors.block_device,
            Self::CharDevice => colors.char_device,
            Self::Directory { .. } => colors.directory,
            Self::File => colors.file,
            Self::Pipe => colors.pipe,
            Self::Socket => colors.socket,
            Self::Symlink { .. } => colors.symlink,
            Self::Unknown => colors.unknown,
        }
    }
    pub fn extra(&self) -> Option<colored::ColoredString> {
        use colored::Colorize;
        match self {
            Self::Directory { .. } => Some("/".white().bold()),
            Self::Pipe => Some("|".white().bold()),
            Self::Socket => Some("-".white().bold()),
            Self::Symlink { target } => Some(format!(" -> {}", target.display()).italic()),
            _ => None,
        }
    }
}
