mod domain;
mod server;
mod settings;
mod state;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment
    dotenv().ok();

    // Initialize the application state
    let settings = settings::Settings::new("APP")?;
    let state = state::AppState { settings };

    // Start the server
    server::serve(state).await?;

    Ok(())
}
