use crate::config::PermColors;
use libc::mode_t;
use std::os::unix::fs::PermissionsExt;

#[derive(Clone, Copy)]
enum PermType {
    User,
    Group,
    Other,
}

impl PermType {
    fn masks(self) -> (mode_t, mode_t, mode_t) {
        use libc::{
            S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
        };
        match self {
            Self::User => (S_IRUSR, S_IWUSR, S_IXUSR),
            Self::Group => (S_IRGRP, S_IWGRP, S_IXGRP),
            Self::Other => (S_IROTH, S_IWOTH, S_IXOTH),
        }
    }
    fn check(self, mode: mode_t) -> (bool, bool, bool) {
        let (read, write, exec) = self.masks();
        (mode & read > 0, mode & write > 0, mode & exec > 0)
    }

    pub fn format(self, mode: mode_t, colors: &PermColors) -> String {
        fn else_dash(
            cond: bool,
            if_true: colored::ColoredString,
            dash_color: crate::config::Color,
        ) -> colored::ColoredString {
            if cond {
                if_true
            } else {
                dash_color.apply("-")
            }
        }

        fn format_rwx((r, w, x): (bool, bool, bool), colors: &PermColors) -> String {
            format!(
                "{}{}{}",
                else_dash(r, colors.read.apply("r"), colors.none),
                else_dash(w, colors.write.apply("w"), colors.none),
                else_dash(x, colors.execute.apply("x"), colors.none),
            )
        }

        format_rwx(self.check(mode), colors)
    }
}

pub fn perms(metadata: &std::fs::Metadata, colors: &PermColors) -> String {
    let mode = metadata.permissions().mode();

    let user = PermType::User.format(mode, colors);
    let group = PermType::Group.format(mode, colors);
    let other = PermType::Other.format(mode, colors);

    [user, group, other].join("")
}
