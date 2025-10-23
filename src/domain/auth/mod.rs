pub mod handler;
pub mod model;
pub mod service;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

use self::handler::{get_current_user, login, refresh_token, register};

/// Create auth routes
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/me", get(get_current_user))
}
