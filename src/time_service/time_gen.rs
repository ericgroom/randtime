extern crate chrono;
extern crate rand;

use chrono::prelude::*;
use rand::prelude::*;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct TimeInfo {
    pub random_time: DateTime<Utc>,
    pub next_generation_time: DateTime<Utc>,
}

impl TimeInfo {
    pub fn new() -> TimeInfo {
        TimeInfo {
            random_time: make_random_time(),
            next_generation_time: TimeInfo::default_next_time(),
        }
    }
    pub fn default_next_time() -> DateTime<Utc> {
        let pacific_offset = 8;
        let today_at_midnight = Utc::today().and_hms(pacific_offset, 0, 0);

        add_to_date(&today_at_midnight, 24, 0)
    }
}

pub fn make_random_time() -> DateTime<Utc> {
    let utc = Utc::now();

    let mut rng = thread_rng();
    let hour: u32 = rng.gen_range(0, 12);
    let minute: u32 = rng.gen_range(0, 60);

    println!("hours: {}, minutes: {}", hour, minute);
    add_to_date(&utc, hour, minute)
}

pub fn add_to_date(date: &DateTime<Utc>, hours: u32, minutes: u32) -> DateTime<Utc> {
    let hours = chrono::Duration::hours(hours as i64);
    let minutes = chrono::Duration::minutes(minutes as i64);
    let new_date = *date + (hours + minutes);

    // discard anything lte seconds
    Utc.ymd(new_date.year(), new_date.month(), new_date.day())
        .and_hms(new_date.hour(), new_date.minute(), 0)
}
