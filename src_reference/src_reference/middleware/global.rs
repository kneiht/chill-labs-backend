use axum::body::Body;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct RequestId(pub uuid::Uuid);

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Authentication middleware
pub async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // Generate and add request ID to extensions
    let request_id = RequestId(uuid::Uuid::new_v4());
    request.extensions_mut().insert(request_id);

    // Continue with the request
    Ok(next.run(request).await)
}
