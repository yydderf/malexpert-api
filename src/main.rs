#[macro_use] extern crate rocket;

mod fairings;
mod services;
mod logging;
mod routes;
mod consts;
mod domain;
mod crypto;
mod api;

use crate::services::clients::malexp::MalexpClient;
use crate::crypto::secret::Secret;

#[get("/")]
fn index() -> &'static str {
    "Hello, malexpert!"
}

#[launch]
fn rocket() -> _ {
    logging::init();

    rocket::build()
        .manage(MalexpClient::new(crate::consts::client::malexp::BASE_URL.as_str()))
        .manage(Secret::init_from_env().expect("HMAC key required"))
        .attach(fairings::cors::Cors)
        .attach(fairings::logger::Logger)
        .mount("/", routes![index, fairings::cors::options])
        .mount("/samples", routes::samples::routes())
        .mount("/pipeline", routes::pipeline::routes())
}
