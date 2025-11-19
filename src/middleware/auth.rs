use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::error::AppError;
use crate::domain::response::Response as ApiResponse;
use crate::entities::users::Model as User;
use crate::state::AppState;

/// Middleware function to authenticate and authorize requests
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract token from Authorization header
    let token = extract_bearer_token(&request).ok_or_else(|| {
        ApiResponse::<()>::failure_unauthorized(
            "Missing or invalid Authorization header",
            Some("Authorization header must be in format: Bearer <token>".to_string()),
        )
        .into_response()
    })?;

    // Verify token and get user
    let user = state
        .user_service
        .verify_token(&token)
        .await
        .map_err(|err| match err {
            AppError::Unauthorized(msg) => {
                ApiResponse::<()>::failure_unauthorized("Authentication failed", Some(msg))
                    .into_response()
            }
            AppError::Forbidden(msg) => {
                ApiResponse::<()>::failure_forbidden("Access forbidden", Some(msg)).into_response()
            }
            _ => {
                ApiResponse::<()>::failure_internal("Internal server error", Some(err.to_string()))
                    .into_response()
            }
        })?;

    // Add user to request extensions
    request.extensions_mut().insert(user);

    // Continue with the next middleware
    Ok(next.run(request).await)
}

// Helper function to extract Bearer token from Authorization header
fn extract_bearer_token(request: &Request) -> Option<String> {
    let auth_header = request.headers().get(AUTHORIZATION)?.to_str().ok()?;

    if auth_header.starts_with("Bearer ") {
        Some(auth_header.trim_start_matches("Bearer ").to_string())
    } else {
        None
    }
}

/// Extension trait to get user from request extensions
#[allow(unused)]
pub trait RequestUserExt {
    fn user(&self) -> Option<&User>;
}

// Implement the RequestUserExt trait for Request
impl RequestUserExt for Request {
    fn user(&self) -> Option<&User> {
        self.extensions().get::<User>()
    }
}
