extern crate chrono;
extern crate rand;

use chrono::prelude::*;
use rand::prelude::*;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn make_random_time() -> DateTime<Utc> {
    let utc = Utc::now();

    let mut rng = thread_rng();
    let hour: u32 = rng.gen_range(0, 24);
    let minute: u32 = rng.gen_range(0, 24);
    Utc.ymd(utc.year(), utc.month(), utc.day())
        .and_hms(hour, minute, 0)
}

fn parse_time(time: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    time.parse::<DateTime<Utc>>()
}

fn log_and_return(e: &dyn Debug) -> DateTime<Utc> {
    println!("{:?}", e);
    make_random_time()
}

pub fn get_random_time() -> DateTime<Utc> {
    let filepath = match env::var("TIME_FILE") {
        Ok(path) => path,
        Err(e) => return log_and_return(&e),
    };
    let file = File::open(Path::new(&filepath));
    match file {
        Ok(f) => read_time(f),
        Err(e) => return log_and_return(&e),
    }
}

fn read_time(file: File) -> DateTime<Utc> {
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);
    match parse_time(&contents) {
        Ok(time) => time,
        Err(e) => log_and_return(&e),
    }
}

pub fn write_random_time() -> std::io::Result<()> {
    let filepath = env::var("TIME_FILE").expect("Cannot find environment variable TIME_FILE");
    let path = Path::new(&filepath);
    let time = make_random_time();
    let time_str = format!("{:?}", time);
    fs::write(path, time_str)
}
