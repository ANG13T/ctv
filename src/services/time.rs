use std::fs;
use std::path::PathBuf;

pub fn time_modified(file: PathBuf, time_format: &str) -> String {
    if file.symlink_metadata().unwrap().modified().is_ok() {
        let naive = chrono::NaiveDateTime::from_timestamp(
            filetime::FileTime::from_last_modification_time(&file.symlink_metadata().unwrap())
                .seconds() as i64,
            0,
        );

        let datetime: chrono::DateTime<chrono::Local> =
            chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
        datetime.format(time_format).to_string()
    } else {
        "00 000 00:00:00".to_string()
    }
}

pub fn time_created(file: PathBuf, time_format: &str) -> String {
    if filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap()).is_some() {
        let naive = chrono::NaiveDateTime::from_timestamp(
            filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap())
                .unwrap()
                .seconds() as i64,
            0,
        );

        let datetime: chrono::DateTime<chrono::Local> =
            chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
        datetime.format(time_format).to_string()
    } else {
        "00 000 00:00:00".to_string()
    }
}

pub fn time_acessed(file: PathBuf, time_format: &str) -> String {
    let meta = fs::metadata(file).unwrap();

    let naive_time = chrono::NaiveDateTime::from_timestamp(
        filetime::FileTime::from_last_access_time(&meta).seconds() as i64,
        0,
    );

    let datetime: chrono::DateTime<chrono::Local> =
        chrono::DateTime::from_utc(naive_time, *chrono::Local::now().offset());

    return datetime.format(time_format).to_string();
}
