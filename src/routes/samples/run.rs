use rocket::{State, serde::json::Json};

use crate::services::clients::malexp::MalexpClient;
use crate::domain::{
    sample::Sample,
    metadata::Metadata,
    usersel::UserSelections,
    json::samples::run::{RunReq, RunResp},
};
use crate::api::response::APIResponse;


#[post("/<sample_id>/run", format = "json", data = "<usersel_json>")]
pub async fn run(
    client: &State<MalexpClient>, sample_id: &str, usersel_json: Json<UserSelections>
) -> APIResponse<RunResp> {
    let usersel: UserSelections = usersel_json.into_inner();
    let sample = Sample::from_id(sample_id);
    let sample_path = sample.binpath.clone();
    let metadata = match Metadata::load(sample.dir.join(crate::consts::path::metadata::FILENAME)) {
        Ok(m) => m,
        Err(_) => { panic!("Should exist"); }
    };

    let body = RunReq {
        sample_path: sample_path,
        sample_hash_val: metadata.sha256_hash,
        user_selections: usersel,
    };
    let sample = Sample::from_id(&sample_id);
    if !sample.exists() {
        return APIResponse::err_not_found("Sample Not Found", "Please try again");
    }
    let runresp = match client.post_run(&body).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}: {}", e.title, e.detail.unwrap_or_else(|| "no detail".to_string()));
            return APIResponse::err_internal("Internal Server Error", "Please try again later");
        }
    };
    // build up request data 
    // post to malexpert pipeline
    // return the job_id
    APIResponse::ok(runresp)
}

