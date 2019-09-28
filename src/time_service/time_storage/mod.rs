use super::time_gen::TimeInfo;
use chrono::prelude::{DateTime, Utc};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod time_storage_error;
use time_storage_error::TimeStorageError;

pub fn read_time_info() -> Result<TimeInfo, TimeStorageError> {
    let file = get_time_file()?;
    let mut reader = BufReader::new(file);
    let mut random_time = String::new();
    let mut next_time = String::new();
    let _ = reader.read_line(&mut random_time);
    let _ = reader.read_line(&mut next_time);
    let time_info = TimeInfo {
        random_time: parse_time(&random_time)?,
        next_generation_time: parse_time(&next_time)?,
    };
    Ok(time_info)
}

pub fn write_time_info(time: &TimeInfo) -> Result<(), TimeStorageError> {
    let filepath = env::var("TIME_FILE")?;
    let path = Path::new(&filepath);
    if !path.exists() {
        if let Err(e) = File::create(path) {
            println!("{:?}", e);
        }
    }
    let time_str = format!("{:?}\n{:?}", time.random_time, time.next_generation_time);
    match fs::write(path, time_str) {
        Ok(_) => Ok(()),
        Err(e) => Result::Err(TimeStorageError::IOErr(e)),
    }
}

fn get_time_file() -> Result<File, TimeStorageError> {
    let filepath = env::var("TIME_FILE")?;
    match File::open(Path::new(&filepath)) {
        Ok(f) => Result::Ok(f),
        Err(e) => Result::Err(TimeStorageError::IOErr(e)),
    }
}

fn parse_time(time: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    time.parse::<DateTime<Utc>>()
}
