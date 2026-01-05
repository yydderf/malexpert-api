use rocket::serde::{Serialize, Deserialize};
use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;
use std::path::{PathBuf, Path};
use std::io::Read;
use std::fs;

use crate::consts::bininfo::FIRST_CHUNK_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BinFormat {
    Elf,
    Pe,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Bitness {
    B16,
    B32,
    B64,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Endianness {
    Little,
    Big,
    Unknown,
}

fn detect_format_from_prefix(prefix: &[u8]) -> BinFormat {
    if prefix.len() >= 4 && &prefix[..4] == b"\x7fELF" {
        return BinFormat::Elf;
    }

    if prefix.len() >= 2 && &prefix[..2] == b"MZ" {
        return BinFormat::Pe;
    }
    BinFormat::Unknown
}

#[derive(Debug, Serialize)]
pub struct BinInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: String,
    pub format: BinFormat,
}

impl BinInfo {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let size = fs::metadata(&path)?.len();
        let mut file = fs::File::open(&path)?;
        let mut context = Context::new(&SHA256);
        let mut buffer = [0u8; 4096];
        let mut first_chunk: Option<Vec<u8>> = None;
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            if first_chunk.is_none() {
                first_chunk = Some(buffer[..FIRST_CHUNK_SIZE].to_vec());
            }
            context.update(&buffer[..count]);
        }
        let digest = context.finish();
        let hash = HEXLOWER.encode(digest.as_ref());
        let format = first_chunk
            .as_deref()
            .map(detect_format_from_prefix)
            .unwrap_or(BinFormat::Unknown);
        Ok(Self { path, size, hash, format })
    }
}
