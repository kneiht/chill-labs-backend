use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{Response, IntoResponse},
    Json,
};

use crate::application::use_cases::auth::CheckAuthUseCaseDto;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::Role;
use crate::state::AppState;

#[derive(Clone)]
pub struct AuthUser(pub crate::entities::User);

pub async fn auth_middleware(
    state: AppState,
    required_role: Option<Role>,
    mut request: Request,
    next: Next,
) -> Response {

    let auth_header = match request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer ")) {
        Some(h) => h,
        None => return UseCaseResponse::<()>::failure_unauthorized("Authorization header missing or invalid", None).into_response(),
    };

    let dto = CheckAuthUseCaseDto {
        token: auth_header.to_string(),
        role_to_check: required_role,
    };

    let use_case = state.use_cases.check_auth_use_case.clone();
    let response = use_case.execute(dto).await;

    if !response.success {
        return response.into_response();
    }

    let user = match response.data {
        Some(u) => u,
        None => return UseCaseResponse::<()>::failure_internal("User data missing", None).into_response(),
    };
    request.extensions_mut().insert(AuthUser(user));

    next.run(request).await
}
