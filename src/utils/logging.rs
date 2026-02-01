use chrono::{DateTime, Datelike, Utc};
use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, never};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::prelude::Result;

fn create_today_string() -> String {
    let today: DateTime<Utc> = Utc::now();
    let date_string: String =
        format!("{:04}_{:02}_{:02}.log", today.year(), today.month(), today.day());
    date_string
}

/// This function instantiates a global instance of tracing with
/// both console and file output, formatted with `.pretty()`.
///
/// # Errors
///
/// - Returns `SetGlobalDefaultError` if this config couldn't be set for global
///   logging
/// - Returns `ParseError` if specified logging directive was invalid
pub fn setup_logger() -> Result<()> {
    let (cwriter, cguard) = non_blocking(std::io::stdout());
    std::mem::forget(cguard);
    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(cwriter)
        .pretty();

    let today_file: String = create_today_string();
    let appender: RollingFileAppender = never("logs", today_file);

    let (fwriter, fguard) = non_blocking(appender);
    std::mem::forget(fguard);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(fwriter)
        .pretty();

    let filter: EnvFilter = EnvFilter::default().add_directive("chat_reader=info".parse()?);
    let sub = tracing_subscriber::registry().with(console_layer).with(file_layer).with(filter);

    tracing::subscriber::set_global_default(sub)?;
    Ok(())
}
