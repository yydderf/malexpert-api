use std::collections::HashMap;

/*
{
  "stages": {
    "analyzer": {
      "models": [{ "id": "default", "label": "Default" }],
      "params": { "max_depth": { "type": "int", "default": 2 } }
    },
    "encoder": {
      "models": [{ "id": "safe-v2", "label": "SAFE v2" }],
      "params": { "device": { "type": "enum", "values": ["cpu","cuda"], "default": "cpu" } }
    }
  },
  "version": "2026-01-09T00:00:00Z"
}
*/
#[derive(Clone, serde::Serialize)]
pub struct Catalog {
    pub stages: HashMap<String, PipelineStage>,
    pub version: String,
}

#[derive(Clone, serde::Serialize)]
pub struct PipelineStage {
    pub models: Vec<ModelInfo>,
    pub params: HashMap<String, ParamSpec>,
}

#[derive(Clone, serde::Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub label: String,
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type")]
pub struct ParamSpec {
    #[serde(rename = "int")]
    Int: i64,
    #[serde(rename = "float")]
    Float: f64,
    #[serde(rename = "bool")]
    Bool: bool,
    #[serde(rename = "string")]
    r#string: String,
    #[serde(rename = "enum")]
    r#enum: Vec<String>,
}
