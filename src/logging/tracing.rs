use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, Layer, filter::filter_fn};
use super::formatter::{CustomFormatter, FileFormatter, ElkFormatter};
use tracing_appender::{non_blocking, rolling::{RollingFileAppender, Rotation}};
use once_cell::sync::OnceCell;
use crate::consts;

static FILE_GUARD: OnceCell<non_blocking::WorkerGuard> = OnceCell::new();

pub fn init() {
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(consts::logging::LOG_FILE_PREFIX)
        .filename_suffix(consts::logging::LOG_FILE_SUFFIX)
        .build(consts::logging::LOG_FILE_DIR)
        .expect("Failed to initialize log file");

    let (non_blocking_appender, guard) = non_blocking(file_appender);
    let _ = FILE_GUARD.set(guard);

    let console_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rocket=info"));
    let file_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rocket=info"));
    let no_rocket_launch = filter_fn(|meta|
        !meta.target().starts_with("rocket::launch")
        && !meta.target().starts_with("rocket::shield")
    );

    let console_layer = tracing_subscriber::fmt::layer()
        .event_format(CustomFormatter)
        .with_filter(console_filter);
    let file_layer = tracing_subscriber::fmt::layer()
        .event_format(CustomFormatter)
        .with_writer(non_blocking_appender)
        .with_filter(file_filter)
        .with_filter(no_rocket_launch);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .try_init()
        .map_err(|e| {
            panic!("Failed to initialize logger: {:?}", e);
        });
}
