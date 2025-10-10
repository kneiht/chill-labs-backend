use chillteacher::server;
use chillteacher::settings::Settings;
use chillteacher::state::AppState;
use dotenv::dotenv;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            // Add env variables from `.env` file
            dotenv().ok();

            // Load settings
            let settings = init_settings()?;

            // Initialize logging
            let _tracing_guard = init_tracing(&settings)?;
            tracing::info!("App configurations: {:#?}", &settings);

            // Initialize app state
            let state = AppState::new(&settings).await?;

            // Run the server
            server::serve(&settings, state).await?;
            Ok::<(), anyhow::Error>(())
        })?;
    Ok(())
}

// This function initializes the settings by calling the `Settings::new` function.
fn init_settings() -> anyhow::Result<Settings> {
    let settings = Settings::new("CHILLTEACHER")?;
    Ok(settings)
}

// This function initializes the tracing subscriber with the specified settings.
fn init_tracing(settings: &Settings) -> anyhow::Result<WorkerGuard> {
    // Default level: INFO. Can be overridden by `settings.logging.log_level` or RUST_LOG env var.
    let default_log_level = "info".to_string();
    let log_level = settings
        .logging
        .log_level
        .clone()
        .unwrap_or_else(|| std::env::var("RUST_LOG").unwrap_or(default_log_level));

    // Default format: PRETTY. Can be overridden by `settings.logging.log_format`.
    let default_log_format = "pretty".to_string();
    let log_format = settings
        .logging
        .log_format
        .clone()
        .unwrap_or(default_log_format);

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
