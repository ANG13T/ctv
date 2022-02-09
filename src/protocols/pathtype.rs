use crate::decorators;
use crate::protocols::colormanager;
use crate::protocols::file::FileStyle;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum PathType {
    Dir,
    Symlink,
    Path,
    Pipe,
    CharD,
    BlockD,
    Socket,
}

impl PathType {
    pub fn new(file: &Path) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut return_val = Vec::new();
        if file.symlink_metadata()?.is_dir() {
            return_val.push(Self::Dir)
        }
        if file.symlink_metadata()?.file_type().is_symlink() {
            return_val.push(Self::Symlink)
        }
        if file.symlink_metadata()?.file_type().is_fifo() {
            return_val.push(Self::Pipe)
        }
        if file.symlink_metadata()?.file_type().is_char_device() {
            return_val.push(Self::CharD)
        }
        if file.symlink_metadata()?.file_type().is_block_device() {
            return_val.push(Self::BlockD)
        }
        if file.symlink_metadata()?.file_type().is_socket() {
            return_val.push(Self::Socket)
        }
        if return_val.is_empty() {
            return_val.push(Self::Path)
        }
        Ok(return_val)
    }

    fn create_letter(&self, letter: &str, file_style: FileStyle) -> String {
        format!(
            "{}{}{}{}",
            self.get_color_for_type(file_style),
            letter,
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset)
        )
    }

    pub fn get_letter_for_type(&self, file_style: FileStyle) -> String {
        match self {
            Self::Dir => self.create_letter("d", file_style),
            Self::Symlink => self.create_letter("l", file_style),
            Self::Pipe => self.create_letter("|", file_style),
            Self::CharD => self.create_letter("c", file_style),
            Self::BlockD => self.create_letter("b", file_style),
            Self::Socket => self.create_letter("s", file_style),
            _ => self.create_letter(".", file_style),
        }
    }

    pub fn get_color_for_type(&self, path_styles: FileStyle) -> String {
        match self {
            Self::Dir => colormanager::colorize_string(&path_styles.dir_color, "".to_string()),
            Self::Symlink => {
                colormanager::colorize_string(&path_styles.symlink_color, "".to_string())
            }
            Self::Path => colormanager::colorize_string(&path_styles.path_color, "".to_string()),
            Self::Pipe => colormanager::colorize_string(&path_styles.pipe_color, "".to_string()),
            Self::CharD => colormanager::colorize_string(&path_styles.chard_color, "".to_string()),
            Self::BlockD => {
                colormanager::colorize_string(&path_styles.blockd_color, "".to_string())
            }
            Self::Socket => {
                colormanager::colorize_string(&path_styles.socket_color, "".to_string())
            }
        }
    }

    pub fn get_text_traits_for_type(&self, name: &str, file: &Path) -> String {
        match self {
            Self::Dir => decorators::bold(&format!(
                "{}{}/",
                name,
                termion::color::Fg(termion::color::White)
            )),
            Self::Symlink => decorators::italic(&format!(
                "{} -> {}",
                name,
                std::fs::read_link(file).unwrap().display().to_string()
            )),
            Self::Path => decorators::bold(name),
            Self::Pipe => decorators::bold(&format!(
                "{}{}",
                name,
                termion::color::Fg(termion::color::White)
            )),
            Self::CharD => decorators::bold(name),
            Self::BlockD => decorators::bold(name),
            Self::Socket => decorators::bold(&format!(
                "{}{}",
                name,
                termion::color::Fg(termion::color::White)
            )),
        }
    }
}
