use rocket::Route;
use std::collections::HashMap;

use crate::domain::pipeline::Catalog;
use crate::api::response::APIResponse;

// GET /pipeline/catalog → available models of all stages
#[get("/catalog")]
pub fn catalog() -> APIResponse<Catalog> {
    APIResponse::ok(Catalog { stages: HashMap::new(), version: "".to_string() })
}

pub fn routes() -> Vec<Route> {
    routes![catalog]
}
