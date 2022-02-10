use crate::protocols;
use crate::protocols::colormanager::colorize_string;
use crate::protocols::file::FileStyle;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

pub fn perms(file: std::path::PathBuf, file_style: FileStyle) -> String {
    let mode = file.symlink_metadata().unwrap().permissions().mode() as u16;
    let user = masking(
        mode,
        S_IRUSR as u16,
        S_IWUSR as u16,
        S_IXUSR as u16,
        file_style.clone(),
    );
    let group = masking(
        mode,
        S_IRGRP as u16,
        S_IWGRP as u16,
        S_IXGRP as u16,
        file_style.clone(),
    );
    let other = masking(
        mode,
        S_IROTH as u16,
        S_IWOTH as u16,
        S_IXOTH as u16,
        file_style.clone(),
    );
    let f = protocols::PathType::new(&file).unwrap()[0].get_letter_for_type(file_style);
    [f, user, group, other].join("")
}

fn masking(mode: u16, read: u16, write: u16, execute: u16, file_style: FileStyle) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => construct_perm_string("-", "-", "-", file_style),
        (_, 0, 0) => construct_perm_string("r", "-", "-", file_style),
        (0, _, 0) => construct_perm_string("-", "w", "-", file_style),
        (0, 0, _) => construct_perm_string("-", "-", "x", file_style),
        (_, 0, _) => construct_perm_string("r", "-", "x", file_style),
        (_, _, 0) => construct_perm_string("r", "w", "-", file_style),
        (0, _, _) => construct_perm_string("-", "w", "x", file_style),
        (_, _, _) => construct_perm_string("r", "w", "x", file_style),
    }
}

fn colorize_perm(input: &str, file_style: FileStyle) -> String {
    if input == "-" {
        return colorize_string(&file_style.dash_color, "-".to_string());
    } else if input == "w" {
        return colorize_string(&file_style.write_color, "w".to_string());
    } else if input == "r" {
        return colorize_string(&file_style.read_color, "r".to_string());
    } else if input == "x" {
        return colorize_string(&file_style.execute_color, "x".to_string());
    }
    colorize_string("RESET", "".to_string())
}

fn construct_perm_string(
    string1: &str,
    string2: &str,
    string3: &str,
    file_style: FileStyle,
) -> String {
    let mut result: String = colorize_perm(string1, file_style.clone());
    result.push_str(&colorize_perm(string2, file_style.clone()));
    result.push_str(&colorize_perm(string3, file_style.clone()));
    result.push_str(&colorize_perm(".", file_style));
    result
}
