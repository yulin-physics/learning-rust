#![feature(proc_macro_hygiene, decl_macro)]

// attributes to indicate importing macros from another crate
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Timestamp {
    time: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time")]
fn time_now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp {
        time: now.to_rfc3339(),
    };
    Json(timestamp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, time_now])
        .launch();
}
