use std::{fs};
use std::path::PathBuf;

pub fn time_modified(file: PathBuf, time_format: &str) -> String{
    let meta = fs::metadata(file).unwrap();

    let naive_time = chrono::NaiveDateTime::from_timestamp(
    filetime::FileTime::from_last_modification_time(&meta).seconds()
    as i64, 0);

    let datetime: chrono::DateTime<chrono::Local> =
      chrono::DateTime::from_utc(naive_time, *chrono::Local::now().offset());

    return datetime.format(time_format).to_string();
}

pub fn time_created(file: PathBuf, time_format: &str) -> String{
    let meta = fs::metadata(file).unwrap();

    let file_time = filetime::FileTime::from_creation_time(&meta);

    if file_time == None {
      println!("FILE TIME IS NONE");
    }



    let naive_time = chrono::NaiveDateTime::from_timestamp(
      file_time.unwrap().seconds()
      as i64, 0);

    let datetime: chrono::DateTime<chrono::Local> =
      chrono::DateTime::from_utc(naive_time, *chrono::Local::now().offset());

    return datetime.format(time_format).to_string();
}

pub fn time_acessed(file: PathBuf, time_format: &str) -> String{
  let meta = fs::metadata(file).unwrap();

  let naive_time = chrono::NaiveDateTime::from_timestamp(
  filetime::FileTime::from_last_access_time(&meta).seconds()
  as i64, 0);

  let datetime: chrono::DateTime<chrono::Local> =
    chrono::DateTime::from_utc(naive_time, *chrono::Local::now().offset());

  return datetime.format(time_format).to_string();
}