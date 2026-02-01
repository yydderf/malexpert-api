use crate::domain::usersel::UserSelections;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RunReq<'a> {
    pub sample_id: &'a str,
    pub sample_hash_val: &'a str,
    pub user_selections: UserSelections,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RunResp {
    pub job_id: String,
}
