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
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    pub stages: HashMap<String, PipelineStage>,
    pub version: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineStage {
    pub models: Vec<ModelInfo>,
    pub params: HashMap<String, ParamSpec>,
    pub description: String,
    pub next: Vec<String>,
    pub default: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub help: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ParamSpec {
    #[serde(rename = "int")] // "type": "Int" -> "type": "int"
    Int { default: i64 },
    #[serde(rename = "bool")]
    Bool { default: bool },
    #[serde(rename = "float")]
    Float { default: f64 },
    #[serde(rename = "string")]
    String { default: String },
    #[serde(rename = "enum")]
    Enum { values: Vec<String>, default: String },
}

