#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod timegen;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value)
    }
    println!("{:?}", timegen::get_random_time());
    rocket::ignite().mount("/", routes![index]).launch();
}
