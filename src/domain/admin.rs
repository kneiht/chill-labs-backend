use crate::AppState;
use axum::Router;
use std::sync::Arc;

use crate::entities::users;
use crud_macros::make_crud_routes;

// Combine all admin routes
pub fn router() -> Router<Arc<AppState>> {
    let user_routes = make_crud_routes!(
        entity: users::Entity,
        model: users::Model,
        active_model: users::ActiveModel,
        path: "/users"
    );

    Router::new().nest("/admin", user_routes)
}
