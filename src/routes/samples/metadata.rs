use crate::domain::sample::Sample;
use crate::domain::metadata::Metadata;
use crate::api::response::APIResponse;

// GET /samples/{id}/metadata → json metadata
#[get("/<sample_id>/metadata")]
pub fn metadata(sample_id: &str) -> APIResponse<Metadata> {
    // let metadata = match Metadata::from_id(&sample_id) {}
    let sample = Sample::from_id(&sample_id);
    let metapath = sample.dir.join(crate::consts::path::metadata::FILENAME);

    if metapath.exists() {
        match Metadata::load(metapath) {
            Ok(m) => { return APIResponse::ok(m); }
            Err(_) => {}
        }
    }

    let metadata = match Metadata::try_from(&sample) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load bin from id {}: {}", &sample_id, e);
            return APIResponse::err_internal("Internal Server Error", "Please try again later")
        }
    };

    // tokio::spawn(async move {
    //     metadata.save(metapath)
    // });

    APIResponse::ok(metadata)
}
