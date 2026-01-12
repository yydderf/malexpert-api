#[macro_use] extern crate rocket;

mod fairings;
mod services;
mod logging;
mod routes;
mod consts;
mod domain;
mod api;

use crate::services::clients::malexp::MalexpClient;

#[get("/")]
fn index() -> &'static str {
    "Hello, malexpert!"
}

#[launch]
fn rocket() -> _ {
    logging::init();

    rocket::build()
        .manage(MalexpClient::new(crate::consts::client::MALEXP_API_BASE_URL.as_str()))
        .attach(fairings::cors::Cors)
        .attach(fairings::logger::Logger)
        .mount("/", routes![index, fairings::cors::options])
        .mount("/samples", routes::samples::routes())
        .mount("/pipeline", routes::pipeline::routes())
}
