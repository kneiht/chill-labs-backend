use anyhow::Context;
use axum::{routing::get, Router};
use std::net::{IpAddr, SocketAddr};

use crate::domain::healthcheck::handler::healthcheck;
use crate::state::AppState;

pub async fn serve(state: AppState) -> anyhow::Result<()> {
    // Routes
    let app = Router::new()
        .route("/api/healthcheck", get(healthcheck))
        .with_state(state.clone());

    // Server host ip
    let host = state
        .settings
        .server
        .host
        .unwrap_or_else(|| "127.0.0.1".to_string());

    // Check if host is valid ip
    let host_ip = host.parse::<IpAddr>().context("Invalid host IP")?;

    // Server port ip
    let port = state.settings.server.port.unwrap_or(3000);

    // Server address
    let addr = SocketAddr::new(host_ip, port);

    // Print server address
    println!("Listening on http://{}", addr);

    // Bind server to address
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind TCP listener")?;

    // Start server
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
