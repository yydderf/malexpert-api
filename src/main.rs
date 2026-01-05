#[macro_use] extern crate rocket;

mod services;
mod routes;
mod consts;
mod domain;
mod logger;
mod cors;

use crate::cors::{Cors, options};
use crate::logger::{ Logger };

#[get("/")]
fn index() -> &'static str {
    "Hello, malexpert!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .attach(Logger)
        .mount("/", routes![index, options])
        .mount("/samples", routes::samples::routes())
}
