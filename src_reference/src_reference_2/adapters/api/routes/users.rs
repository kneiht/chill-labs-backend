use crate::adapters::api::handlers::{
    create_user, delete_user_by_id, get_user_by_id, get_users, update_user,
};
use crate::adapters::api::middleware::auth_middleware;
use crate::entities::Role;
use crate::state::AppState;
use axum::{
    Router,
    handler::Handler,
    middleware,
    routing::{get, post},
};

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_user.layer(middleware::from_fn({
                let state = state.clone();
                move |req, next| auth_middleware(state.clone(), Some(Role::ADMIN), req, next)
            })))
            .get(get_users), // No auth for getting users list
        )
        .route(
            "/{id}",
            get(get_user_by_id.layer(middleware::from_fn({
                let state = state.clone();
                move |req, next| auth_middleware(state.clone(), Some(Role::ADMIN), req, next)
            })))
            .put(update_user.layer(middleware::from_fn({
                let state = state.clone();
                move |req, next| auth_middleware(state.clone(), Some(Role::ADMIN), req, next)
            })))
            .delete(delete_user_by_id.layer(middleware::from_fn({
                let state = state.clone();
                move |req, next| auth_middleware(state.clone(), Some(Role::ADMIN), req, next)
            }))),
        )
}
