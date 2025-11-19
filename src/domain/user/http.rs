use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use std::sync::Arc;

use super::model::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::domain::error::ToResponse;
use crate::entities::users::Model as User;
use crate::state::AppState;

/// Register a new user
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    state
        .user_service
        .register(req)
        .await
        .to_response_created("User registered successfully")
}

/// Login user
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    state
        .user_service
        .login(req)
        .await
        .to_response("Login successful")
}

/// Refresh access token
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    state
        .user_service
        .refresh_token(req)
        .await
        .to_response("Token refreshed successfully")
}

/// Get current user profile
pub async fn me(Extension(user): Extension<User>) -> impl IntoResponse {
    crate::domain::response::Response::success_ok(user, "User profile retrieved successfully")
}

/// User/Auth Router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/me", get(me))
}
