use anyhow::Context;
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/healthcheck", get(healthcheck));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TCP listener")?;
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}