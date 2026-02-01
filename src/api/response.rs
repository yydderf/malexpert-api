use rocket::http::{ContentType, Status, Header};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::serde_json;
// add readable trait (Read) to String
use std::io::Cursor;

use crate::api::error::APIErrorBody;
use crate::api_error_ctor;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum APIBody<T> {
    Ok(T),
    Err(APIErrorBody),
}

// struct User { id: u32, name: String } -> APIResponse<User>
pub struct APIResponse<T> {
    status: Status,
    body: APIBody<T>,
}

// constructor
impl<T> APIResponse<T> {
    // APIResponse::ok(User { id: 1234, name: "Fred".into() }
    pub fn ok(body: T) -> Self {
        Self {
            status: Status::Ok,
            body: APIBody::Ok(body),
        }
    }
    // APIResponse::with_status(Status::..., User { id: 1234, name: "Fred".into() }
    pub fn with_status(status: Status, body: T) -> Self {
        Self {
            status,
            body: APIBody::Ok(body),
        }
    }

    pub fn err(status: Status, title: impl Into<String>) -> Self {
        // title.into()
        Self {
            status,
            body: APIBody::Err(APIErrorBody{ title: title.into(), detail: None }),
        }
    }
    pub fn err_with_detail(status: Status, title: impl Into<String>, detail: impl Into<String>) -> Self {
        // title.into() / detail.into()
        Self {
            status,
            body: APIBody::Err(APIErrorBody{ title: title.into(), detail: Some(detail.into()) })
        }
    }
    // APIResponse::internal(...);
    // APIResponse::bad_request(...);
    // APIResponse::unsupported(...);
    api_error_ctor!(err_internal,    Status::InternalServerError);
    api_error_ctor!(err_bad_request, Status::BadRequest);
    api_error_ctor!(err_unsupported, Status::UnsupportedMediaType);
    api_error_ctor!(err_not_found,   Status::NotFound);
}

// defines how to convert APIError to HTTP response 
// response owns the data -> 'static
impl<'r, T> Responder<'r, 'static> for APIResponse<T> 
where
    T: serde::Serialize,
{
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

pub(crate) struct ExtHeader<S>(pub S, pub Vec<Header<'static>>);

impl<'r, 'o: 'r, R> Responder<'r, 'o> for ExtHeader<R>
where
    R: Responder<'r, 'o> 
{
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let response = self.0.respond_to(req)?;
        let mut new_response = Response::build_from(response);
        for header in self.1 {
            new_response.header(header);
        }
        new_response.ok()
    }
}

