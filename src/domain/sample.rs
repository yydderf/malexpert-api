use crate::consts;
use crate::domain::bininfo::BinInfo;

use uuid::Uuid;
use std::path::PathBuf;
use std::io;

#[derive(Debug)]
pub struct Sample {
    pub id: String,
    pub dir: PathBuf,
    pub binpath: PathBuf,
    pub analyzed: bool,
}

impl Sample {
    pub fn create() -> Self {
        let id = Uuid::new_v4().to_string();
        Self::from_id(&id)
    }
    pub fn from_id(id: &str) -> Self {
        let dir = PathBuf::from(consts::upload::DIR).join(&id);
        let binpath = dir.join("binary");
        Self { id: id.to_string(), dir, binpath, analyzed: false }
    }
    pub fn load_bin(&self) -> io::Result<BinInfo> {
        BinInfo::from_path(&self.binpath)
    }
    pub fn exists(&self) -> bool {
        self.dir.is_dir()
    }
}

