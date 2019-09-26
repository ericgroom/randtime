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

#[derive(Debug)]
pub struct TimeInfo {
    random_time: DateTime<Utc>,
    next_generation_time: DateTime<Utc>,
}

impl TimeInfo {
    fn new() -> TimeInfo {
        TimeInfo {
            random_time: make_random_time(),
            next_generation_time: TimeInfo::default_next_time(),
        }
    }
    fn default_next_time() -> DateTime<Utc> {
        let now = Utc::now();

        // ensures that this is never before the generated random time
        add_to_date(&now, 12, 1)
    }
}

fn make_random_time() -> DateTime<Utc> {
    let utc = Utc::now();

    let mut rng = thread_rng();
    let hour: u32 = rng.gen_range(0, 12);
    let minute: u32 = rng.gen_range(0, 60);

    println!("hours: {}, minutes: {}", hour, minute);
    add_to_date(&utc, hour, minute)
}

fn add_to_date(date: &DateTime<Utc>, hours: u32, minutes: u32) -> DateTime<Utc> {
    let hours = chrono::Duration::hours(hours as i64);
    let minutes = chrono::Duration::minutes(minutes as i64);
    let new_date = *date + (hours + minutes);

    // discard anything lte seconds
    Utc.ymd(new_date.year(), new_date.month(), new_date.day())
        .and_hms(new_date.hour(), new_date.minute(), 0)
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
    let time_info = match file {
        Ok(f) => read_time_info(f),
        Err(e) => {
            println!("{:?}", e);
            generate_and_persist_time_info()
        }
    };
    if Utc::now() > time_info.next_generation_time {
        generate_and_persist_time_info().random_time
    } else {
        time_info.random_time
    }
}

fn generate_and_persist_time_info() -> TimeInfo {
    println!("generating new times");
    let new_info = TimeInfo::new();
    write_time_info(&new_info);
    new_info
}

fn read_time_info(file: File) -> TimeInfo {
    let mut reader = BufReader::new(file);
    let mut random_time = String::new();
    let mut next_time = String::new();
    let _ = reader.read_line(&mut random_time);
    let _ = reader.read_line(&mut next_time);
    let next_time = match parse_time(&next_time) {
        Ok(time) => time,
        Err(e) => {
            println!("{:?}", e);
            Utc::now()
        }
    };
    let random_time = match parse_time(&random_time) {
        Ok(time) => time,
        Err(e) => log_and_return(&e),
    };
    TimeInfo {
        random_time: random_time,
        next_generation_time: next_time,
    }
}

pub fn write_time_info(time: &TimeInfo) {
    let filepath = env::var("TIME_FILE").expect("Cannot find environment variable TIME_FILE");
    let path = Path::new(&filepath);
    if !path.exists() {
        if let Err(e) = File::create(path) {
            println!("{:?}", e);
        }
    }
    let time_str = format!("{:?}\n{:?}", time.random_time, time.next_generation_time);
    let result = fs::write(path, time_str);
    if let Some(err) = result.err() {
        println!("{:?}", err);
    }
}
