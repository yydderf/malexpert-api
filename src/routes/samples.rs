use rocket::Route;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::Form;

use crate::consts;

// use services::storage::save_file;
// use services::analysis::run_pipeline;

#[derive(serde::Serialize)]
pub struct UploadResp {
    filename: String,
    size: u64,
}

#[post("/upload/form", data = "<file>")]
pub async fn upload_binary_form(mut file: Form<TempFile<'_>>) -> Result<Json<UploadResp>, Status> {
    // interact w/ svelte
    // save_file();
    // preprocess();
    // run_pipeline();
    let filename = file
        .name()
        .map(|s| s.to_string())
        .unwrap_or("unknown.bin".into());

    let size = file.len();

    file.persist_to(format!("{}/{}", consts::upload::DIR, filename))
        .await
        .map_err(|e| {
            error!("Failed to persist uploaded file {}: {}", filename, e);
            Status::InternalServerError
        })?;

    Ok(Json(UploadResp { filename, size }))
}

// #[post("/upload/raw", data = "<data>")]
// pub async fn upload_binary_raw(data: Data<'_>) -> Result<Status, Status> {
//     // interact w/ curl:
//     // curl -XPOST -H "Content-Type: application/octet-stream"
//     // --data-binary <file> <url>
// }

pub fn routes() -> Vec<Route> {
//     routes![upload_binary_form, upload_binary_raw]
    routes![upload_binary_form]
}
