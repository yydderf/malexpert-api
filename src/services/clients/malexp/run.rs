use rocket::serde::json::Json;

use crate::api::error::APIErrorBody;
use crate::services::clients::malexp::MalexpClient;
use crate::domain::json::samples::run::{RunResp, RunReq};

impl MalexpClient {
    pub async fn post_run(&self, runreq: &RunReq) -> Result<RunResp, APIErrorBody> {
        self.post_json::<RunReq, RunResp>(crate::consts::client::malexp::RUN, runreq).await
    }
}
