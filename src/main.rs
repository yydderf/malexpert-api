#[macro_use] extern crate rocket;

mod cors;
mod routes;
mod consts;

use crate::cors::{Cors, options};

#[get("/")]
fn index() -> &'static str {
    "Hello, malexpert!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![index, options])
        .mount("/samples", routes::samples::routes())
}
