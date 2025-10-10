use axum::{extract::State, response::IntoResponse};

use crate::application::use_cases::UseCase;
use crate::application::use_cases::auth::{LoginUseCaseDto, RegisterUseCaseDto};
use crate::state::AppState;

pub async fn login(
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<LoginUseCaseDto>,
) -> impl IntoResponse {
    let use_case = state.use_cases.login_use_case.clone();
    use_case.execute(dto).await
}

pub async fn register(
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<RegisterUseCaseDto>,
) -> impl IntoResponse {
    let use_case = state.use_cases.register_use_case.clone();
    use_case.execute(dto).await
}
