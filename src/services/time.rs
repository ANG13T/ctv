use crate::config::TimeType;
use filetime::FileTime;
use std::fs::Metadata;

pub fn get_modified(metadata: &Metadata) -> FileTime {
    FileTime::from_last_modification_time(metadata)
}

pub fn get_created(metadata: &Metadata) -> FileTime {
    FileTime::from_creation_time(metadata).unwrap_or(FileTime::zero())
}

pub fn get_accessed(metadata: &Metadata) -> FileTime {
    FileTime::from_last_access_time(metadata)
}

pub fn get(metadata: &Metadata, time_type: TimeType) -> FileTime {
    match time_type {
        TimeType::Accessed => get_accessed(metadata),
        TimeType::Created => get_created(metadata),
        TimeType::Modified => get_modified(metadata),
    }
}

pub fn format(time: &FileTime, format: &str) -> String {
    let time = chrono::NaiveDateTime::from_timestamp(time.seconds(), time.nanoseconds());
    let time = <chrono::DateTime<chrono::Local>>::from_utc(time, *chrono::Local::now().offset());
    time.format(format).to_string()
}
