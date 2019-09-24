extern crate chrono;
extern crate rand;

use chrono::prelude::*;
use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path;

pub fn make_random_time() -> DateTime<Utc> {
    let utc = Utc::now();

    let mut rng = thread_rng();
    let hour: u32 = rng.gen_range(0, 24);
    let minute: u32 = rng.gen_range(0, 24);
    Utc.ymd(utc.year(), utc.month(), utc.day())
        .and_hms(hour, minute, 0)
}

pub fn parse_time(time: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    time.parse::<DateTime<Utc>>()
}

pub fn get_random_time() -> DateTime<Utc> {
    let filepath = env::var("TIME_FILE");
    let file = filepath.as_ref().map(path::Path::new).map(File::open);

    // bleh, cannot find a way to flatten
    match file {
        Ok(f) => match f {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);
                parse_time(&contents).unwrap_or(make_random_time())
            }
            Err(_) => make_random_time(),
        },
        Err(_) => make_random_time(),
    }
}

pub fn write_random_time() -> std::io::Result<()> {
    let filepath = env::var("TIME_FILE").unwrap();
    let path = path::Path::new(&filepath);
    let time = make_random_time();
    let time_str = format!("{:?}", time);
    std::fs::write(path, time_str)
}
