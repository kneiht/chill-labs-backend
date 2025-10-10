use crate::domain::v1::healthcheck::handler as healthcheck_handler;
use crate::domain::v1::user::handler as auth_handler;
use crate::middleware::auth_middleware;
use crate::middleware::request_id_middleware;

use crate::state::AppState;

use axum::routing::{get, post, put};
use axum::{middleware, Router};

// Function to initialize the Rest API routes
pub fn build_restapi_routes(state: AppState) -> Router {
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/healthcheck", get(healthcheck_handler::healthcheck))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/signup", post(auth_handler::signup))
        .route("/auth/verify-email", get(auth_handler::verify_email))
        .route_layer(middleware::from_fn(request_id_middleware));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/auth/me", get(auth_handler::get_current_user))
        .route("/auth/change-password", put(auth_handler::change_password))
        .route(
            "/auth/resend-verification",
            post(auth_handler::resend_verification_email),
        )
        .route_layer(middleware::from_fn(request_id_middleware))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Combine public and protected routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state.clone())
}
