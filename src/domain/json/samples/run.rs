use std::path::PathBuf;

use crate::domain::usersel::UserSelections;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RunReq {
    pub sample_path: PathBuf,
    pub sample_hash_val: String,
    pub user_selections: UserSelections,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RunResp {
    pub job_id: i32,
}
