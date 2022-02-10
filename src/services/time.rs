use filetime::FileTime;
use std::fs::Metadata;

pub fn time_modified(metadata: &Metadata, time_format: &str) -> String {
    let time = filetime::FileTime::from_last_modification_time(metadata);
    format_time(&time, time_format)
}

pub fn time_created(metadata: &Metadata, time_format: &str) -> String {
    let time = filetime::FileTime::from_creation_time(metadata);
    let time = time.unwrap_or(filetime::FileTime::zero());
    format_time(&time, time_format)
}

pub fn time_accessed(metadata: &Metadata, time_format: &str) -> String {
    let time = filetime::FileTime::from_last_access_time(metadata);
    format_time(&time, time_format)
}

fn format_time(time: &FileTime, format: &str) -> String {
    let time = chrono::NaiveDateTime::from_timestamp(time.seconds(), time.nanoseconds());
    let time = <chrono::DateTime<chrono::Local>>::from_utc(time, *chrono::Local::now().offset());
    time.format(format).to_string()
}
