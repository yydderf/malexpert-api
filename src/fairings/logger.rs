use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data};

pub struct Logger;

#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Logger",
            kind: Kind::Request,
        }

    }

    async fn on_request(&self, _req: &mut Request<'_>, _: &mut Data<'_>) {
        // println!(
        //     "Incoming from {}: {}, {}",
        //     req.client_ip().map(|ip| ip.to_string()).unwrap_or_else(|| "<unknown>".into()), req.method(), req.uri()
        // );
    }
}
