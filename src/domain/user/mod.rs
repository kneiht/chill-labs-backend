pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use crate::middleware::{require_admin, require_teacher_or_admin};
use crate::state::AppState;
use axum::middleware;
use axum::routing::{get, post, put};
use axum::Router;

use self::handler::{create_user, delete_user, get_all_users, get_user, update_user};

pub fn user_routes() -> Router<AppState> {
    use axum::routing::delete as delete_method;

    let admin_only = Router::new()
        .route("/", post(create_user))
        .route("/{id}", put(update_user))
        .route("/{id}", delete_method(delete_user))
        .layer(middleware::from_fn(require_admin));

    let teacher_or_admin = Router::new()
        .route("/", get(get_all_users))
        .layer(middleware::from_fn(require_teacher_or_admin));

    let authenticated = Router::new().route("/{id}", get(get_user));

    Router::new()
        .merge(admin_only)
        .merge(teacher_or_admin)
        .merge(authenticated)
}
