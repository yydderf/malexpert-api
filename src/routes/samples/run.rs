use rocket::{State, serde::json::Json};

use crate::services::clients::malexp::MalexpClient;
use crate::domain::{
    sample::Sample,
    signed::Signed,
    metadata::Metadata,
    usersel::UserSelections,
    json::samples::run::{RunReq, RunResp},
};
use crate::api::response::APIResponse;
use crate::consts::path;


#[post("/<sample_id>/run", format = "json", data = "<usersel_json>")]
pub async fn run(
    client: &State<MalexpClient>, sample_id: &str, usersel_json: Json<UserSelections>
) -> APIResponse<RunResp> {
    let usersel: UserSelections = usersel_json.into_inner();
    let sample = Sample::from_id(sample_id);

    if !sample.exists() {
        return APIResponse::err_not_found("Sample Not Found", "Please try again");
    }

    let metadata = match Signed::<Metadata>::load(sample.dir.join(path::metadata::FILENAME)) {
        Ok(p) => p.into_payload(),
        Err(_) => { return APIResponse::err_not_found("File Not Found", "Please upload the sample again"); }
    };

    let body = RunReq {
        sample_id: sample_id,
        sample_hash_val: &metadata.sha256_hash,
        user_selections: usersel,
    };

    let runresp = match client.post_run(&body).await {
        Ok(v) => v,
        Err(_) => {
            // error!("{}: {}", e.title, e.detail.unwrap_or_else(|| "no detail".to_string()));
            return APIResponse::err_internal("Internal Server Error", "Please try again later");
        }
    };
    // build up request data 
    // post to malexpert pipeline
    // return the job_id
    APIResponse::ok(runresp)
}

