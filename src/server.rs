use anyhow::Context;
use axum::{routing::get, Router};
use crate::domain::healthcheck::handler::healthcheck;

pub async fn serve() -> anyhow::Result<()> {
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