use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::Json;

use super::model::{
    AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
    UserInfo,
};
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::state::AppState;

/// Handler for user registration
/// POST /api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Response<AuthResponse> {
    state
        .auth_service
        .register(req)
        .await
        .to_response_created("User registered successfully")
}

/// Handler for user login
/// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Response<AuthResponse> {
    state
        .auth_service
        .login(req)
        .await
        .to_response("Login successful")
}

/// Handler for token refresh
/// POST /api/auth/refresh
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Response<RefreshTokenResponse> {
    state
        .auth_service
        .refresh_token(req)
        .await
        .map(|token| RefreshTokenResponse { token })
        .to_response("Token refreshed successfully")
}

/// Handler to get current user information from token
/// GET /api/auth/me
pub async fn get_current_user(
    State(state): State<AppState>,
    request: Request,
) -> Response<UserInfo> {
    // Extract token from Authorization header
    let token = match extract_bearer_token(&request) {
        Some(t) => t,
        None => {
            return Response::failure_unauthorized(
                "Missing or invalid Authorization header",
                Some("Authorization header must be in format: Bearer <token>".to_string()),
            );
        }
    };

    // Verify token and get user
    state
        .auth_service
        .verify_and_get_user(&token)
        .await
        .map(|user| user.into())
        .to_response("User retrieved successfully")
}

/// Helper function to extract Bearer token from Authorization header
fn extract_bearer_token(request: &Request) -> Option<String> {
    let auth_header = request.headers().get(AUTHORIZATION)?.to_str().ok()?;

    if auth_header.starts_with("Bearer ") {
        Some(auth_header.trim_start_matches("Bearer ").to_string())
    } else {
        None
    }
}
