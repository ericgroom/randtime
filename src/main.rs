#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value)
    }
    rocket::ignite().mount("/", routes![index]).launch();
}
