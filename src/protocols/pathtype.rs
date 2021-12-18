use std::path::{Path};
use std::os::unix::fs::FileTypeExt;
use crate::decorators;

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
      if file.symlink_metadata()?.is_dir() {return_val.push(Self::Dir) }
      if file.symlink_metadata()?.file_type().is_symlink() {return_val.push(Self::Symlink)}
      if file.symlink_metadata()?.file_type().is_fifo() {return_val.push(Self::Pipe)}
      if file.symlink_metadata()?.file_type().is_char_device() {return_val.push(Self::CharD)}
      if file.symlink_metadata()?.file_type().is_block_device() {return_val.push(Self::BlockD)}
      if file.symlink_metadata()?.file_type().is_socket() {return_val.push(Self::Socket)}
      if return_val.is_empty() {return_val.push(Self::Path)}
  
      Ok(return_val)
    }
  
    fn create_letter(&self, letter: &str) -> String {
      format!(
        "{}{}{}{}",
        self.get_color_for_type(),
        letter,
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
      )
    }
  
    pub fn get_letter_for_type(&self) -> String {
      match self {
        Self::Dir     => self.create_letter("d"),
        Self::Symlink => self.create_letter("l"),
        Self::Pipe    => self.create_letter("|"),
        Self::CharD   => self.create_letter("c"),
        Self::BlockD  => self.create_letter("b"),
        Self::Socket  => self.create_letter("s"),
        _             => self.create_letter("."),
      }
    }
  
    pub fn get_color_for_type(&self) -> String {
      match self {
        Self::Dir     => format!("{}", termion::color::Fg(termion::color::LightBlue)),
        Self::Symlink => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
        Self::Path    => format!("{}", termion::color::Fg(termion::color::White)),
        Self::Pipe    => format!("{}", termion::color::Fg(termion::color::Yellow)),
        Self::CharD   => format!("{}{}", termion::color::Bg(termion::color::Yellow), termion::color::Fg(termion::color::LightBlue)),
        Self::BlockD  => format!("{}", termion::color::Fg(termion::color::LightGreen)),
        Self::Socket  => format!("{}", termion::color::Fg(termion::color::LightRed)),
      }
    }
  
    pub fn get_text_traits_for_type(&self, name: &str, file: &Path) -> String {
      match self {
        Self::Dir     => decorators::bold(&format!( "{}{}/", name, termion::color::Fg(termion::color::White))),
        Self::Symlink => decorators::italic(&format!( "{} -> {}", name, std::fs::read_link(file).unwrap().display().to_string())),
        Self::Path    => decorators::bold(name),
        Self::Pipe    => decorators::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
        Self::CharD   => decorators::bold(name),
        Self::BlockD  => decorators::bold(name),
        Self::Socket  => decorators::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
      }
    }
  }