use crate::adapters::api::handlers::{
    create_post, delete_post_by_id, get_post_by_id, get_posts, update_post,
};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post).get(get_posts))
        .route(
            "/{id}",
            get(get_post_by_id)
                .put(update_post)
                .delete(delete_post_by_id),
        )
}
