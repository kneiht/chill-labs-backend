use crate::adapters::api::handlers::auth::{login, register};
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
