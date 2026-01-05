use tracing::{Event, Level};
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields},
    registry::LookupSpan,
};
use std::fmt::Write;
use time::OffsetDateTime;
use time_tz::{OffsetDateTimeExt, timezones};

pub(super) struct LevelPrefixFormatter;

impl<S, N> fmt::FormatEvent<S, N> for LevelPrefixFormatter
where 
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &fmt::FmtContext<'_, S, N>, mut writer: fmt::format::Writer<'_>, event: &Event<'_>) -> std::fmt::Result {
        let ts = OffsetDateTime::now_utc()
            .to_timezone(timezones::db::asia::TAIPEI)
            .format(crate::consts::logging::TS_FMT)
            .unwrap();
        let prefix = match *event.metadata().level() {
            Level::ERROR => "[-] ",
            Level::WARN  => "[!] ",
            Level::INFO  => "[+] ",
            Level::DEBUG => "[.] ",
            Level::TRACE => "[~] ",
        };
        // writer.write_fmt("{} {}", &ts, preifx)?;
        writer.write_str(&ts)?;
        writer.write_char(' ')?;
        writer.write_str(prefix)?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
