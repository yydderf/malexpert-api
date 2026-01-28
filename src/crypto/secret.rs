use std::env;

use anyhow::{Context, Result};
use base64::Engine;
use ring::hmac;

use crate::domain::signed::Signed;

// TODO
// fn validate_key_len
// wrap HMAC key into a struct to be managed by rocket
pub struct Secret {
    key: hmac::Key,
}


impl Secret {
    pub fn init_from_env() -> Result<Self> {
        let s = env::var(crate::consts::secret::MALEXP_HMAC_KEY_ENV)
            .with_context(|| format!(
                "HMAC key is not set ({})", crate::consts::secret::MALEXP_HMAC_KEY_ENV
            ))?;

        let raw_key = base64::engine::general_purpose::STANDARD
            .decode(s.trim())
            .with_context(|| format!(
                "env {} is not base64 decodable", crate::consts::secret::MALEXP_HMAC_KEY_ENV
            ))?;

        let key = hmac::Key::new(hmac::HMAC_SHA256, &raw_key);

        Ok(Self { key })
    }

    fn _sign(&self, bytes: &[u8]) -> String {
        let tag = hmac::sign(&self.key, bytes);
        data_encoding::HEXLOWER.encode(tag.as_ref())
    }

    pub fn sign<S>(&self, src: S) -> anyhow::Result<Signed<S>>
    where
        S: serde::Serialize,
    {
        let bytes = postcard::to_allocvec(&src)
            .context("postcard serialize failed")?;

        let tag_hex = self._sign(&bytes);

        Ok(Signed { payload: src, tag_hex })
    }

    pub fn validate<S>(&self, signed: &Signed<S>) -> anyhow::Result<()>
    where
        S: serde::Serialize,
    {
        let expected = signed.tag_hex();

        let payload = signed.payload();
        let bytes = postcard::to_allocvec(&payload)
            .context("postcard serialize failed")?;
        let tag_hex = self._sign(&bytes);

        anyhow::ensure!(expected == tag_hex, "signature mismatched");

        Ok(())
    }
}
