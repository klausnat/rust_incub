use std::time::SystemTime;
use std::{fs::File, sync::Arc};

use tracing::{debug, error, info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{fmt, prelude::*, Layer};

use chrono::{DateTime, Utc};

// Custom timer for nanosecond precision
struct NanoTimer;

impl FormatTime for NanoTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        write!(w, "{}", datetime.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true))
    }
}


fn main() {
    setup_logging().unwrap();

    debug!(
        lvl = "DEBUG",
        file = "app.log",
        message = "some debug information",
        method = "POST",
        path = "http://hello.com"
    );

    info!(
        lvl = "INFO",
        file = "app.log",
        message = "some info message",
        method = "POST",
        path = "http://hello.com"
    );

    error!(
        lvl = "ERROR",
        file = "app.log",
        message = "some Error message",
    );

    warn!(
        lvl = "WARNING",
        file = "app.log",
        message = "some warning message",
    );
}

// Main function to set up both loggers
pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create the access.log file appender
    let access_log = File::create("access.log")?;
    let access_log = Arc::new(access_log);

    let timer = NanoTimer;

    // Main app loggers: WARN+ to STDERR, others to STDOUT
    // 1. STDOUT layer: INFO and DEBUG levels
    let stdout_layer = fmt::layer()
        .json()
        .with_timer(timer)
        .with_writer(std::io::stdout)
        .with_filter(LevelFilter::INFO) // Minimum level: INFO
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // Only allow INFO and DEBUG levels to STDOUT
            *metadata.level() <= tracing::Level::INFO
        }));

    // 2. STDERR layer: WARN and ERROR levels
    let stderr_layer = fmt::layer()
        .json()
        .with_timer(NanoTimer)
        .with_writer(std::io::stderr)
        .with_filter(LevelFilter::WARN) // Minimum level: WARN
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // Only allow WARN and ERROR levels to STDERR
            *metadata.level() >= tracing::Level::WARN
        }));

    // 3. Access logger: all logs to access.log file
    let access_layer = fmt::layer()
        .json()
        .with_timer(NanoTimer)
        .with_writer(move || access_log.clone())
        .with_filter(tracing_subscriber::filter::filter_fn(|_metadata| {
            true // Log everything to access.log for now
        }));

    // Combine both layers and initialize the subscriber
    let subscriber = tracing_subscriber::registry()
        .with(stdout_layer)
        .with(stderr_layer)
        .with(access_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
