use rocket::response::stream::EventStream;
use crate::api::error::APIErrorBody;
use crate::services::clients::malexp::MalexpClient;
use crate::consts::client::malexp;

impl MalexpClient {
    pub async fn get_events(&self, job_id: &str, event_id: &str) -> Result<EventStream![], APIErrorBody> {
        let path = format!("{}/{}{}", malexp::JOBS, job_id, malexp::EVENTS);
        self.get_event_stream(&path, event_id).await
    }
}
