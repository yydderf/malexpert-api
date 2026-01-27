use std::path::PathBuf;

use uuid::Uuid;

#[derive(Debug)]
pub struct Sample {
    pub id: String,
    pub dir: PathBuf,
    pub binpath: PathBuf,
}

impl Sample {
    pub fn create() -> Self {
        let id = Uuid::new_v4().to_string();
        Self::from_id(&id)
    }
    pub fn from_id(id: &str) -> Self {
        let dir = PathBuf::from(crate::consts::upload::DIR).join(&id);
        let binpath = dir.join("binary");
        Self { id: id.to_string(), dir, binpath }
    }
    pub fn exists(&self) -> bool {
        self.dir.is_dir()
    }
}

