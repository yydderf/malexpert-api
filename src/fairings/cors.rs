use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};

use crate::consts::cors::ALLOWED_ORIGINS;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if let Some(origin) = req.headers().get_one("Origin") {
            if ALLOWED_ORIGINS.contains(&origin) {
                res.set_header(Header::new("Access-Control-Allow-Origin", origin));
                res.set_header(Header::new("Access-Control-Allowed-Credentials", "true"));
                res.set_header(Header::new("Vary", "Origin"));
            }
        }
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

// handles preflight
// returns NoContent for every option request
#[options("/<_..>")]
pub fn options() -> Status {
    Status::NoContent
}
