use anyhow::Context;
use axum::http::Method;
use axum::{routing::get, Router};
use std::net::{IpAddr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use crate::domain::auth::auth_routes;
use crate::domain::healthcheck::handler::healthcheck;
use crate::domain::note::note_routes;
use crate::domain::user::user_routes;

use crate::state::AppState;

pub async fn serve(state: &AppState) -> anyhow::Result<()> {
    // CORS setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    // Routes
    let app = Router::new()
        .route("/api/healthcheck", get(healthcheck))
        .nest("/api/auth", auth_routes())
        .nest("/api/users", user_routes())
        .nest("/api/notes", note_routes())
        .with_state(state.clone())
        .layer(cors);

    // Server host ip
    let host = state
        .settings
        .server
        .host
        .clone()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    // Check if host is valid ip
    let host_ip = host.parse::<IpAddr>().context("Invalid host IP")?;

    // Server port ip
    let port = state.settings.server.port.unwrap_or(3000);

    // Server address
    let addr = SocketAddr::new(host_ip, port);

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
