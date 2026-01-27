use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryType {
    Executable,
    Library,
    Object,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub enum BinaryKind {
    PE,
    Elf,
    Mach,
    Unknown,
}

