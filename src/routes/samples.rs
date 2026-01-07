use rocket::Route;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::Form;
use rocket::response::Responder;
use std::path::{Path, PathBuf};
use std::{io::Read, fs::{self, File}};

use crate::domain::sample::Sample;
use crate::domain::bininfo::{Metadata, BinaryKind};

#[derive(serde::Serialize)]
pub struct UploadResp {
    sample_id: String,
}

#[derive(serde::Serialize)]
pub struct APIError {
    title: String,
    detail: String,
}

#[derive(Responder)]
pub enum UploadResponse {
    #[response(status = 200, content_type = "json")]
    Ok(Json<UploadResp>),

    #[response(status = 400, content_type = "json")]
    Unsupported(Json<APIError>),

    #[response(status = 415, content_type = "json")]
    BadRequest(Json<APIError>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<APIError>),
}

fn upload_preprocess() -> Result<(String, PathBuf), Status> {
    let sample = Sample::create();

    fs::create_dir_all(&sample.dir)
        .map_err(|e| {
            error!("Failed to create directory for sample {}: {}", sample.id, e);
            Status::InternalServerError
        })?;

    Ok((sample.id, sample.binpath))
}

fn lazy_validate_binary(path: impl AsRef<Path>) -> std::io::Result<BinaryKind> {
    let mut f = File::open(path)?;
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf)?;

    // Object::Elf | Object::PE | Object::Mach
    Ok(match buf {
        [0x7F, b'E', b'L', b'F'] => BinaryKind::Elf,
        [b'M', b'Z', ..]         => BinaryKind::PE,
        [0xFE, 0xED, 0xFA, 0xCE] |
        [0xCE, 0xFA, 0xED, 0xFE] |
        [0xFE, 0xED, 0xFA, 0xCF] |
        [0xCF, 0xFA, 0xED, 0xFE] => BinaryKind::Mach,
        _ => BinaryKind::Unknown,
    })
}

#[post("/upload/form", data = "<file>")]
pub async fn upload_binary_form(mut file: Form<TempFile<'_>>) -> UploadResponse {
    if file.len() == 0 {
        return UploadResponse::BadRequest(Json(APIError {
            title: "Empty upload".into(),
            detail: "Please try again".into(),
        }));
    }

    let tmp_path = match file.path() {
        Some(p) => p,
        None => {
            return UploadResponse::Internal(Json(APIError {
                title: "Internal Server Error".into(),
                detail: "Please try again later".into(),
            }))
        }
    };

    match lazy_validate_binary(tmp_path) {
        Ok(BinaryKind::Elf | BinaryKind::PE | BinaryKind::Mach) => {}
        Ok(_) | Err(_) => {
            return UploadResponse::Unsupported(Json(APIError {
                title: "Unsupported file type".into(),
                detail: "Please upload a valid ELF/PE binary".into(),
            }))
        }
    }

    let (sample_id, sample_path) = match upload_preprocess() {
        Ok(v) => v,
        Err(_) => {
            return UploadResponse::Internal(Json(APIError {
                title: "Internal Server Error".into(),
                detail: "Please try again later".into(),
            }))
        }
    };

    if file.persist_to(&sample_path).await.is_err() {
        return UploadResponse::Internal(Json(APIError {
            title: "Internal Server Error".into(),
            detail: "Please try again later".into(),
        }));
    }

    UploadResponse::Ok(Json(UploadResp { sample_id }))
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
pub fn get_metadata(sample_id: &str) -> Result<Json<Metadata>, Status> {
    let metadata = Sample::from_id(&sample_id)
        .load_bin()
        .map_err(|e| {
            error!("Failed to load bin from id {}: {}", &sample_id, e);
            Status::InternalServerError
        })?;
    Ok(Json(metadata))
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
