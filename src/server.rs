use anyhow::Context;
use axum::body::Body;
use axum::http::{header, Method, Response, StatusCode, Uri};
use axum::middleware;
use axum::response::{IntoResponse, Json};
use axum::routing::get;
use axum::{extract::Path, Router};
use std::net::{IpAddr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use crate::domain::auth::auth_routes;
use crate::domain::healthcheck::healthcheck_routes;
use crate::domain::note::note_routes;
use crate::domain::user::user_routes;
use crate::middleware::auth_middleware;

use serde_json::json;

use rust_embed::RustEmbed;

use crate::state::AppState;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

/// Handler to serve static assets embedded in the binary.
async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    match Assets::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap(),
    }
}

/// Fallback handler for 404 Not Found errors.
async fn fallback(uri: Uri) -> impl IntoResponse {
    let message = format!("Route '{}' not found", uri.path());
    let body = Json(json!({ "success": false, "message": message, "data": null }));

    (StatusCode::NOT_FOUND, body)
}

/// Serve the application routes
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

    // Protected routes (require authentication)
    let protected_routes = Router::new()
        .nest("/users", user_routes())
        .nest("/notes", note_routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Public routes
    let app = Router::new()
        .nest("/healthcheck", healthcheck_routes())
        .nest("/auth", auth_routes())
        // Serve static files from the embedded assets
        .route(
            "/admin",
            get(|| async { static_handler(Path("admin.html".to_string())).await }),
        )
        .route(
            "/note-app",
            get(|| async { static_handler(Path("notes.html".to_string())).await }),
        )
        .route(
            "/tester",
            get(|| async { static_handler(Path("api.html".to_string())).await }),
        )
        .merge(protected_routes)
        .fallback(fallback)
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
