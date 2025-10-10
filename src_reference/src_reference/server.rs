use crate::domain::v1::routes::build_restapi_routes as build_restapi_v1_routes;
use crate::settings::Settings;
use crate::state::AppState;
use anyhow::Context;

use axum::http::{header, HeaderMap, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get; // Import 'get'
use axum::Router;
use std::net::{IpAddr, SocketAddr};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

// Function to start the server
pub async fn serve(settings: &Settings, state: AppState) -> anyhow::Result<()> {
    // Configure CORS to allow localhost with any port
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

    // Initialize the router with the provided state
    let router = init_router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Retrieve the server settings and define the address to listen on
    let port = settings
        .server
        .port
        .clone()
        .context("Server port is not set")?;
    let host_str = settings
        .server
        .host
        .clone()
        .context("Server host is not set")?;
    let host = host_str.parse::<IpAddr>()?; // Parse the host string into an IpAddr
    let addr = SocketAddr::new(host, port);

    // Start the server
    tracing::info!("REST API v1 available at http://{}/api/v1/", &addr);
    tracing::info!("Testapi available at http://{}/testapi", &addr);
    tracing::info!("Listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok::<(), anyhow::Error>(())
}

// Handler function to serve the testapi.html file
async fn test_api_handler() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/testapi.html").await {
        Ok(body) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
            (StatusCode::OK, headers, body).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not read testapi.html: {}", err),
        )
            .into_response(),
    }
}

// Function to initialize the router
fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/testapi", get(test_api_handler)) // Add route for the test page
        .nest("/api/v1", build_restapi_v1_routes(state.clone())) // Keep existing API routes
}
