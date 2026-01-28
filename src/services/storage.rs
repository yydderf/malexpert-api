use std::{fs, path::Path};

use anyhow::{Context, Result};
use rocket::serde::json::serde_json;

use crate::domain::signed::Signed;

impl<T> Signed<T>
where
    T: serde::Serialize,
{
    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let tmp_path = path.with_extension(crate::consts::path::TMP_EXT);
        let bytes = serde_json::to_vec(self)
            .context("serialize Signed<T> as json")?;
        fs::write(&tmp_path, &bytes)
            .with_context(|| format!("write to {}", tmp_path.display()))?;
        fs::rename(&tmp_path, path)
            .with_context(|| format!("rename {} to {}", tmp_path.display(), path.display()))?;

        Ok(())
    }
}

impl<T> Signed<T>
where
    T: for<'de> serde::Deserialize<'de>, // DeserializeOwned
{
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let bytes = fs::read(path)
            .with_context(|| format!("read from {}", path.display()))?;
        let v = serde_json::from_slice(&bytes)
            .context("deserialize Signed<T> from json")?;

        Ok(v)
    }
}
