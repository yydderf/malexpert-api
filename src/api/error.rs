#[derive(serde::Serialize, serde::Deserialize)]
pub struct APIErrorBody {
    pub title: String,
    pub detail: Option<String>,
}

// pub struct APIError {
//     status: Status,
//     body: APIErrorBody,
// }

// APIResponse::err() / APIResponse::err_with_code()

// impl APIError {
//     pub fn new(status: Status, title: impl Into<String>, detail: impl Into<String>) -> Self {
//         Self { 
//             status,
//             body: APIErrorBody {
//                 title: title.into(),
//                 detail: detail.into(),
//             }
//         }
//     }
// 
//     pub fn unsupported(title: impl Into<String>, detail: impl Into<String>) -> Self {
//         Self::new(Status::UnsupportedMediaType, title, detail)
//     }
//     pub fn bad_request(title: impl Into<String>, detail: impl Into<String>) -> Self {
//         Self::new(Status::BadRequest, title, detail)
//     }
//     pub fn internal(title: impl Into<String>, detail: impl Into<String>) -> Self {
//         Self::new(Status::InternalServerError, title, detail)
//     }
//     pub fn not_found(title: impl Into<String>, detail: impl Into<String>) -> Self {
//         Self::new(Status::NotFound, title, detail)
//     }
// }

