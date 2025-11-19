use crate::AppState;
use axum::Router;
use std::sync::Arc;

use crate::entities::{lessons, notes, sentences, users, word_sentences, words};
use crud_macros::make_crud_routes;

// Combine all admin routes
pub fn router() -> Router<Arc<AppState>> {
    let user_routes = make_crud_routes!(
        entity: users::Entity,
        model: users::Model,
        active_model: users::ActiveModel,
        path: "/users"
    );

    let lesson_routes = make_crud_routes!(
        entity: lessons::Entity,
        model: lessons::Model,
        active_model: lessons::ActiveModel,
        path: "/lessons"
    );

    let note_routes = make_crud_routes!(
        entity: notes::Entity,
        model: notes::Model,
        active_model: notes::ActiveModel,
        path: "/notes"
    );

    let sentence_routes = make_crud_routes!(
        entity: sentences::Entity,
        model: sentences::Model,
        active_model: sentences::ActiveModel,
        path: "/sentences"
    );

    let word_routes = make_crud_routes!(
        entity: words::Entity,
        model: words::Model,
        active_model: words::ActiveModel,
        path: "/words"
    );

    let word_sentence_routes = make_crud_routes!(
        entity: word_sentences::Entity,
        model: word_sentences::Model,
        active_model: word_sentences::ActiveModel,
        path: "/word_sentences"
    );

    Router::new().nest(
        "/admin",
        user_routes
            .merge(lesson_routes)
            .merge(note_routes)
            .merge(sentence_routes)
            .merge(word_routes)
            .merge(word_sentence_routes),
    )
}
