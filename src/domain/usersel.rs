use std::collections::HashMap;
use crate::domain::pipeline::ParamSpec;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserSelections {
    pub stages: Vec<StageSelection>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StageSelection {
    pub model: String,
    pub params: HashMap<String, ParamSpec>,
    pub stage: String,
}
