use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

pub fn group(metadata: &Metadata) -> String {
    let group = users::get_group_by_gid(metadata.gid());
    if let Some(g) = group {
        String::from(g.name().to_string_lossy())
    } else {
        String::from(" ")
    }
}

pub fn user(metadata: &Metadata) -> String {
    let user = users::get_user_by_uid(metadata.uid());
    if let Some(u) = user {
        String::from(u.name().to_string_lossy())
    } else {
        String::from(" ")
    }
}
