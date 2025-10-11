mod domain;
mod middleware;
mod server;
mod settings;
mod state;
mod utils;

use dotenv::dotenv;
use settings::Settings;
use state::AppState;
use utils::tracing::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment
    dotenv().ok();

    // Initialize the application state
    let settings = Settings::new("APP")?;

    // Initialize logging
    let _tracing_guard = init_tracing(&settings)?;
    tracing::info!("App configurations: {:#?}", &settings);

    // Initialize app state
    let state = AppState::new(&settings).await?;

    // Start the server
    server::serve(&state).await?;
    tracing::info!("Server started {:?}", &state.settings.server);
    Ok(())
}
