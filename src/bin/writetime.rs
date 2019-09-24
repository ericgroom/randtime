extern crate dotenv;

use randtime::timegen::write_random_time;

fn main() {
    dotenv::dotenv().ok();
    write_random_time();
}
