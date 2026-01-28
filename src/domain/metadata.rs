use std::{io::Read, path::{Path, PathBuf}, fs, convert::{TryFrom, Into}};

use rocket::serde::{json::serde_json, Serialize, Deserialize};
use goblin::Object;
use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};

use crate::domain::sample::Sample;
use crate::domain::bininfo::{BinaryType, BinaryKind};

fn shannon_entropy(counts: &[u64; 256], total: u64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for &c in counts {
        if c == 0 {
            continue;
        }
        let p = c as f64 / total as f64;
        entropy -= p * p.log2();
    }
    entropy
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub size: u64,
    pub sha256_hash: String,
    pub entropy: f64,
    pub arch: Option<String>,
    pub bitness: Option<u8>,
    pub endianness: Option<String>,
    pub exec_type: BinaryType,
}

impl TryFrom<&Sample> for Metadata {
    type Error = anyhow::Error;
    fn try_from(sample: &Sample) -> anyhow::Result<Self> {
        let path = &sample.binpath;
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let mut file = fs::File::open(&path)?;
        let mut context = Context::new(&SHA256);

        let mut byte_freq = [0u64; 256];
        let mut total = 0u64;
        let mut buffer = [0u8; 4096];

        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
            for &b in &buffer[..count] {
                byte_freq[b as usize] += 1;
            }
            total += count as u64;
        }
        let entropy = shannon_entropy(&byte_freq, total);
        let sha256_hash = HEXLOWER.encode(context.finish().as_ref());

        let buf = fs::read(&path)?;
        // might crush if binary is 16-bit
        let (arch, bitness, endianness, exec_type) = match Object::parse(&buf).ok() {
            Some(Object::Elf(elf)) => {
                let arch = Some("ELF".into());
                let bitness = Some(if elf.is_64 { 64 } else { 32 });
                let endianness = Some(if elf.little_endian { "Little".into() } else { "Big".into() });
                let exec_type = match elf.header.e_type {
                    goblin::elf::header::ET_EXEC => BinaryType::Executable,
                    goblin::elf::header::ET_DYN => BinaryType::Library,
                    goblin::elf::header::ET_REL => BinaryType::Object,
                    _ => BinaryType::Unknown,
                };
                (arch, bitness, endianness, exec_type)
            }
            Some(Object::PE(pe)) => {
                let arch = Some("PE".into());
                let bitness = Some(if pe.is_64 { 64 } else { 32 });
                let endianness = Some("Little".into());
                let exec_type = if pe.is_lib { BinaryType::Library } else { BinaryType::Executable };
                (arch, bitness, endianness, exec_type)
            }
            Some(Object::Mach(_)) => (Some("Mach-O".into()), None, None, BinaryType::Executable),
            _ => (None, None, None, BinaryType::Unknown),
        };

        Ok(Self { 
            size,
            sha256_hash,
            entropy,
            arch,
            bitness,
            endianness,
            exec_type,
        })
    }
}
