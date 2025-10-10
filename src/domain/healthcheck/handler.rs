use crate::domain::response::UseCaseResponse;
use serde_json::json;

pub async fn healthcheck() -> UseCaseResponse<serde_json::Value> {
    UseCaseResponse::success_ok(json!({"status": "ok"}), "Health check successful")
}
