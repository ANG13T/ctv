use std::{fs};
use std::path::PathBuf;

pub fn time_modified(file: PathBuf, time_format: &str) -> String{
    let meta = fs::metadata(file).unwrap();

    let naive_time = chrono::NaiveDateTime::from_timestamp(
    filetime::FileTime::from_last_modification_time(&meta).seconds()
    as i64, 0);

    let datetime: chrono::DateTime<chrono::Local> =
      chrono::DateTime::from_utc(naive_time, *chrono::Local::now().offset());
    let result = datetime.format(time_format).to_string();

    println!("{}", result);

    return result;
}

pub fn time_created(file: PathBuf) -> String{
    let meta = fs::metadata(file).unwrap();
    let mtime = FileTime::from_last_access_time(&meta).unix_seconds();
    println!("{}", mtime);
    return "".to_string()
}