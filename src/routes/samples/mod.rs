use rocket::Route;

mod run;
mod upload;
mod metadata;

// TODO
// GET /samples/{id}/libraries → imported libs/APIs
// #[get("/<sample_id>/libraries")]
// GET /samples/{id}/artifacts → list what’s available
// #[get("/<sample_id>/artifacts")]

pub fn routes() -> Vec<Route> {
    routes![
        run::run,
        metadata::get_metadata,
        upload::upload_binary_form,
    ]
}
