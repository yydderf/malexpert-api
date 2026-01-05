use rocket::Route;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::Form;
use std::path::PathBuf;
use std::fs;

use crate::domain::sample::Sample;
use crate::domain::bininfo::{BinFormat, Bitness, Endianness, BinInfo};

#[derive(serde::Serialize)]
pub struct UploadResp {
    sample_id: String,
}

#[derive(serde::Serialize)]
pub struct Metadata {
    sample_id: String,
    hash: String,
    size: u64,
    format: BinFormat,
    bit: Bitness,
    endianness: Endianness,
}

fn upload_preprocess() -> Result<(String, PathBuf), Status> {
    let sample = Sample::create();

    fs::create_dir_all(&sample.dir)
        .map_err(|e| {
            error!("Failed to create directory for sample {}: {}", sample.id, e);
            Status::InternalServerError
        })?;

    println!("{sample:#?}");

    Ok((sample.id, sample.binpath))
}

#[post("/upload/form", data = "<file>")]
pub async fn upload_binary_form(mut file: Form<TempFile<'_>>) -> Result<Json<UploadResp>, Status> {
    let (sample_id, sample_path) = upload_preprocess()?;
    file.persist_to(&sample_path)
        .await
        .map_err(|e| {
            error!("Failed to persist uploaded file {}", e);
            Status::InternalServerError
        })?;

    let bininfo = Sample::from_id(&sample_id)
        .load_bin()
        .map_err(|e| {
            error!("Failed to load bin from id {}: {}", &sample_id, e);
            Status::InternalServerError
        })?;

    println!("{bininfo:#?}");

    Ok(Json(UploadResp { sample_id }))
}

// #[post("/upload/raw", data = "<data>")]
// pub async fn upload_binary_raw(data: Data<'_>) -> Result<Status, Status> {
//     // interact w/ curl:
//     // curl -XPOST -H "Content-Type: application/octet-stream"
//     // --data-binary <file> <url>
// }

#[post("/<sample_id>/analyze")]
pub fn analyze(sample_id: String) -> Result<Status, Status> {
    let sample = Sample::from_id(&sample_id);
    if sample.exists() { Ok(Status::Ok) } else { Ok(Status::NotFound) }
}

// GET /samples/{id}/metadata → json metadata
#[get("/<sample_id>/metadata")]
pub fn get_metadata(sample_id: String) -> Result<Json<BinInfo>, Status> {
    let bininfo = Sample::from_id(&sample_id)
        .load_bin()
        .map_err(|e| {
            error!("Failed to load bin from id {}: {}", &sample_id, e);
            Status::InternalServerError
        })?;
    Ok(Json(bininfo))
}

// GET /samples/{id}/libraries → imported libs/APIs
// #[get("/<sample_id>/libraries")]
// GET /samples/{id}/artifacts → list what’s available
// #[get("/<sample_id>/artifacts")]

pub fn routes() -> Vec<Route> {
//     routes![upload_binary_form, upload_binary_raw]
    routes![
        upload_binary_form,
        analyze,
        get_metadata,
    ]
}
