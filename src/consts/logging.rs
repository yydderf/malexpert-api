use time::macros::format_description;
use time::format_description::BorrowedFormatItem;

pub const TS_FMT: &[BorrowedFormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub const ERROR: &'static str = "[-] ";
pub const WARN:  &'static str = "[!] ";
pub const INFO:  &'static str = "[+] ";
pub const DEBUG: &'static str = "[.] ";
pub const TRACE: &'static str = "[~] ";

pub const LOG_FILE_DIR:  &'static str = "logs";
pub const LOG_FILE_PREFIX: &'static str = "server";
pub const LOG_FILE_SUFFIX: &'static str = "log";
