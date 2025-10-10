use crate::adapters::api::handlers::{create_image, get_images};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn image_routes() -> Router<AppState> {
    Router::new().route("/", post(create_image).get(get_images))
}
