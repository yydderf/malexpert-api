use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::serde_json;
// add readable trait (Read) to String
use std::io::Cursor;

#[derive(serde::Serialize)]
pub struct APIErrorBody {
    title: String,
    detail: String,
}

pub struct APIError {
    status: Status,
    body: APIErrorBody,
}

impl APIError {
    pub fn new(status: Status, title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { 
            status,
            body: APIErrorBody {
                title: title.into(),
                detail: detail.into(),
            }
        }
    }

    pub fn unsupported(title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::UnsupportedMediaType, title, detail)
    }
    pub fn bad_request(title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::BadRequest, title, detail)
    }
    pub fn internal(title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::InternalServerError, title, detail)
    }
    pub fn not_found(title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::NotFound, title, detail)
    }
}

// defines how to convert APIError to HTTP response 
// response owns the data -> 'static
impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self.body)
            .map_err(|_| Status::InternalServerError)?;
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
