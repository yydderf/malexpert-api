use rocket::response::stream::EventStream;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Header;
use rocket::State;

use crate::consts::client::malexp::events;
use crate::services::clients::malexp::MalexpClient;
use crate::api::response::ExtHeader;

pub struct LastEventId(Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LastEventId {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let id = request.headers().get_one(events::LAST_EVENT_ID).map(|s| s.to_string());
        Outcome::Success(LastEventId(id))
    }
}

#[get("/<job_id>/events")]
pub async fn proxy_job_events(
    client: &State<MalexpClient>, job_id: &str, last_id: LastEventId
) -> ExtHeader<EventStream![]> {
    let id = last_id.0.as_deref().unwrap_or("$");
    let stream = client.get_events(job_id, id)
        .await
        .expect("should get events");
    let headers = vec![
        Header::new("Connection", "keep-alive"),
        Header::new("X-Accel-Buffering", "no"),
    ];
    // stream
    ExtHeader(stream, headers)
}
