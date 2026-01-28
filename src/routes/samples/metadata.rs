use rocket::State;

use crate::{
    domain::{sample::Sample, metadata::Metadata, signed::Signed},
    api::response::APIResponse,
    crypto::secret::Secret,
};

// GET /samples/{id}/metadata → json metadata
#[get("/<sample_id>/metadata")]
pub fn get_metadata(sample_id: &str, secret: &State<Secret>) -> APIResponse<Metadata> {
    // let metadata = match Metadata::from_id(&sample_id) {}
    let sample = Sample::from_id(&sample_id);
    let metapath = sample.dir.join(crate::consts::path::metadata::FILENAME);

    let span = tracing::span!(tracing::Level::INFO, "metadata_span");
    let _guard = span.enter();

    if metapath.exists() {
        if let Ok(signed) = Signed::<Metadata>::load(&metapath) {
            if secret.validate(&signed).is_ok() {
                tracing::debug!("cache hit");
                return APIResponse::ok(signed.into_payload());
            } else {
                tracing::debug!("signature mismatched");
            }
        }
    }

    tracing::debug!("generating new metadata");

    let metadata = match Metadata::try_from(&sample) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load bin from id {}: {}", &sample_id, e);
            return APIResponse::err_internal("Internal Server Error", "Please try again later")
        }
    };

    // tokio::spawn(async move {});
    let signed_metadata = secret.sign::<Metadata>(metadata).expect("should be able to sign");

    signed_metadata.save(metapath).unwrap();

    APIResponse::ok(signed_metadata.into_payload())
}
