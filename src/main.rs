#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate serde;

use dotenv::dotenv;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;

mod time_service;
use time_service::TimeInfo;

#[derive(Serialize)]
struct TemplateContext {
    time_str: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext {
        time_str: format!("{:?}", time_service::get_random_time().random_time),
    };
    Template::render("index", context)
}

#[get("/api/current")]
fn api_current() -> Json<TimeInfo> {
    let current_info = time_service::get_random_time();
    Json(current_info)
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, api_current])
        .mount("/public", StaticFiles::from("./static"))
        .launch();
}
