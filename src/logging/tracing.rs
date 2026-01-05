use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use super::formatter::LevelPrefixFormatter;

pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rocket=info"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().event_format(LevelPrefixFormatter))
        .try_init()
        .map_err(|e| {
            panic!("Failed to initialize logger: {:?}", e);
        });
}
