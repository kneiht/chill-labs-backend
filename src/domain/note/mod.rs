pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn note_routes() -> Router<AppState> {
    use self::handler::{create, delete, get as get_handler, get_all, update};

    Router::new()
        .route("/", post(create).get(get_all))
        .route("/{id}", get(get_handler).put(update).delete(delete))
}
