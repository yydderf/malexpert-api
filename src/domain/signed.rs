use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signed<T> {
    pub payload: T,
    pub tag_hex: String,
}

impl<T> Signed<T> {
    pub fn into_payload(self) -> T {
        self.payload
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn tag_hex(&self) -> &str {
        &self.tag_hex
    }
}
