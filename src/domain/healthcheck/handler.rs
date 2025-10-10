use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}