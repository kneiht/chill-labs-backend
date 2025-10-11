use crate::domain::response::Response;
use serde_json::json;

pub async fn healthcheck() -> Response<serde_json::Value> {
    Response::success_ok(json!({"status": "ok"}), "Health check successful")
}
