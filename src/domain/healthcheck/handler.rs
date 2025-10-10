use crate::domain::response::ApiResponse;
use serde_json::json;

pub async fn healthcheck() -> ApiResponse<serde_json::Value> {
    ApiResponse::success_ok(json!({"status": "ok"}), "Health check successful")
}
