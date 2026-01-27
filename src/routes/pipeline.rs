use std::collections::HashMap;

use rocket::{Route, State};
use tracing::error;

use crate::domain::pipeline::Catalog;
use crate::api::response::APIResponse;
use crate::services::clients::malexp::MalexpClient;

// GET /pipeline/catalog → available models of all stages
#[get("/catalog")]
pub async fn catalog(client: &State<MalexpClient>) -> APIResponse<Catalog> {
    let catalog = match client.get_catalog().await {
        Ok(v) => v,
        Err(e) => {
            error!("{}: {}", e.title, e.detail.unwrap_or_else(|| "no detail".to_string()));
            return APIResponse::err_internal("Internal Server Error", "Please try again later");
        }
    };
    APIResponse::ok(catalog)
}

pub fn routes() -> Vec<Route> {
    routes![catalog]
}
