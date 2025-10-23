pub mod handler;

use self::handler::healthcheck;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

pub fn healthcheck_routes() -> Router<AppState> {
    Router::new().route("/", get(healthcheck))
}
