// Import the tracing_subscriber crate
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Import the Settings struct
use crate::settings::Settings;

// This function initializes the tracing subscriber with the specified settings.
pub fn init_tracing(settings: &Settings) -> anyhow::Result<WorkerGuard> {
    // Get log level from settings
    let log_level = settings.logging.log_level.clone();

    // Get log format from settings
    let log_format = settings.logging.log_format.clone();

    // Configure the writer: Non-blocking stdout
    // In production, logs often go to stdout/stderr and are collected by infrastructure (like Kubernetes).
    // tracing_appender creates a non-blocking writer to avoid blocking application threads.
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    // Build the formatting layer based on the configured format
    let format_layer = fmt::layer().with_writer(non_blocking_writer);
    let format_layer = match log_format.to_lowercase().as_str() {
        "json" => format_layer.json().boxed(),
        _ => format_layer.pretty().boxed(), // Default to pretty
    };

    // Initialize the subscriber with the configured log level and format
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new(log_level))
        .with(format_layer);

    // Initialize the subscriber
    subscriber.init();

    // Return the guard and keep it alive in main
    Ok(guard)
}
