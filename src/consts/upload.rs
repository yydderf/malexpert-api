use rocket::data::ByteUnit;

pub const DIR: &str = "/tmp/malexpert_uploads";

pub const MAX_BIN_SIZE_MB: u64 = 20;
pub const SIZE_LIMIT: ByteUnit = ByteUnit::Megabyte(MAX_BIN_SIZE_MB);
