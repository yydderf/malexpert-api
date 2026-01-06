use tracing::{Event, Level};
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields},
    registry::LookupSpan,
};
use tracing_log::NormalizeEvent;
use std::fmt::Write;
use time::OffsetDateTime;
use time_tz::{OffsetDateTimeExt, timezones};

pub(super) struct CustomFormatter;
pub(super) struct FileFormatter;
pub(super) struct ElkFormatter;

trait CustomFormat {
    fn custom_format(self) -> &'static str;
}

impl CustomFormat for Level {
    fn custom_format(self) -> &'static str {
        match self {
            Level::ERROR => crate::consts::logging::ERROR,
            Level::WARN  => crate::consts::logging::WARN,
            Level::INFO  => crate::consts::logging::INFO,
            Level::DEBUG => crate::consts::logging::DEBUG,
            Level::TRACE => crate::consts::logging::TRACE,
        }
    }
}

impl<S, N> fmt::FormatEvent<S, N> for CustomFormatter
where 
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &fmt::FmtContext<'_, S, N>, mut writer: fmt::format::Writer<'_>, event: &Event<'_>) -> std::fmt::Result {
        let nmetadata = event.normalized_metadata();
        let metadata = nmetadata.as_ref().unwrap_or_else(|| event.metadata());
        let ts = OffsetDateTime::now_utc()
            .to_timezone(timezones::db::asia::TAIPEI)
            .format(crate::consts::logging::TS_FMT)
            .unwrap();
        let level = metadata.level().custom_format();
        let target = metadata.target();
        writer.write_str(&ts)?;
        writer.write_char(' ')?;
        writer.write_str(level)?;
        writer.write_str(target)?;
        writer.write_char(' ')?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
