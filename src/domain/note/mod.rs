pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn note_routes() -> Router<AppState> {
    use self::handler::{create_note, delete_note, get_all_notes, get_note, update_note};

    Router::new()
        .route("/", post(create_note).get(get_all_notes))
        .route("/{id}", get(get_note).put(update_note).delete(delete_note))
}
