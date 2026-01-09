#[macro_use] extern crate rocket;

mod fairings;
mod services;
mod logging;
mod routes;
mod consts;
mod domain;
mod api;

#[get("/")]
fn index() -> &'static str {
    "Hello, malexpert!"
}

#[launch]
fn rocket() -> _ {
    logging::init();

    rocket::build()
        .attach(fairings::cors::Cors)
        .attach(fairings::logger::Logger)
        .mount("/", routes![index, fairings::cors::options])
        .mount("/samples", routes::samples::routes())
}
