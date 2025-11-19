use crate::domain::response::Response;
use crate::AppState;
use axum::{routing::get, Router};
use serde_json::json;
use std::sync::Arc;

pub async fn healthcheck() -> Response<serde_json::Value> {
    Response::success_ok(json!({"server": "ok"}), "Health check successful")
}

/// Healthcheck Router
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(healthcheck))
}
