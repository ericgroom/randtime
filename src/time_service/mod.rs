use chrono::Utc;

mod time_gen;
mod time_storage;

pub use time_gen::TimeInfo;
use time_storage::{read_time_info, write_time_info};

pub fn get_random_time() -> TimeInfo {
    let time_info = match read_time_info() {
        Ok(time_info) => time_info,
        Err(e) => {
            println!("{:?}", e);
            generate_and_persist_time_info()
        }
    };
    if Utc::now() >= time_info.next_generation_time {
        generate_and_persist_time_info()
    } else {
        time_info
    }
}

pub fn generate_and_persist_time_info() -> TimeInfo {
    println!("generating new times");
    let new_info = TimeInfo::new();
    let result = write_time_info(&new_info);
    if let Err(e) = result {
        println!("{:?}", e);
    }
    new_info
}
