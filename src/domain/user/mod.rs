pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

use self::handler::{create_user, delete_user, get_all_users, get_user, update_user};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_all_users))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}
