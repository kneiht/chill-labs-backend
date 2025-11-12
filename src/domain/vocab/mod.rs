pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn vocab_routes() -> Router<AppState> {
    use self::handler::{create_vocab, delete_vocab, get_all_vocabs, get_vocab, update_vocab};

    Router::new()
        .route("/", post(create_vocab).get(get_all_vocabs))
        .route("/{id}", get(get_vocab).put(update_vocab).delete(delete_vocab))
}