use axum::body::Body;
use axum::{
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

use axum_extra::extract::cookie::CookieJar;

use uuid::Uuid;

use crate::state::AppState;
use crate::utils::errors::AppError;

use super::global::RequestId;

// Extract token from Authorization header
fn extract_token(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}

// Authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // Get request_id from Extension
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .cloned()
        .map(|r| r)
        .unwrap_or(RequestId(uuid::Uuid::new_v4()));

    request.extensions_mut().insert(request_id.clone());
    let cookies = CookieJar::from_headers(request.headers());

    // Try to get token from cookie first
    let token = cookies
        .get("auth_token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            // Fall back to Authorization header
            request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
                .and_then(extract_token)
                .map(|t| t.to_string())
        })
        .ok_or_else(|| AppError::Unauthorized {
            request_id: request_id.clone(),
            message: "Missing authentication token".to_string(),
        })?;

    // Get the JWT utility
    let jwt_util = state.jwt_util.as_ref();

    // Verify the token
    let claims = jwt_util
        .verify_token(&token)
        .map_err(|_| AppError::Unauthorized {
            request_id: request_id.clone(),
            message: "Failed to verify authentication token".to_string(),
        })?;

    // Parse the user ID
    let user_id = claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| AppError::Unauthorized {
            request_id: request_id.clone(),
            message: "Invalid user ID in token".to_string(),
        })?;

    // Add the user ID to the request extensions
    request.extensions_mut().insert(user_id);

    // Continue with the request
    Ok(next.run(request).await)
}
