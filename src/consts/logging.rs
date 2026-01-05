use time::macros::format_description;
use time::format_description::BorrowedFormatItem;

pub const TS_FMT: &[BorrowedFormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
