use anyhow::Context;
use axum::http::Method;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::{IpAddr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use crate::domain::auth::handler::{login, register};
use crate::domain::healthcheck::handler::healthcheck;
use crate::domain::note::handler::{
    create_note, delete_note, get_all_notes, get_note_by_id, get_notes_by_user_id, update_note,
};
use crate::domain::user::handler::{
    create_user, delete_user, get_all_users, get_user, update_user,
};
use crate::middleware::auth::AuthUser;
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
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/{id}", put(update_user))
        .route("/api/users/{id}", delete(delete_user))
        // Note routes
        .route("/api/notes", post(create_note))
        .route("/api/notes", get(get_all_notes))
        .route("/api/notes/{id}", get(get_note_by_id))
        .route("/api/notes/{id}", put(update_note))
        .route("/api/notes/{id}", delete(delete_note))
        .route("/api/users/{user_id}/notes", get(get_notes_by_user_id))
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
