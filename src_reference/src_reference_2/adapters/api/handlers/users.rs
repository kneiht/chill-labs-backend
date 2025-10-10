use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::application::use_cases::UseCase;
use crate::entities::{CreateUserDto, UpdateUserDto};
use crate::state::AppState;

pub async fn create_user(
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<CreateUserDto>,
) -> impl IntoResponse {
    let use_case = state.use_cases.add_user_use_case.clone();
    use_case.execute(dto).await
}

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let use_case = state.use_cases.get_all_users_use_case.clone();
    use_case.execute(()).await
}

pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let use_case = state.use_cases.get_user_by_id_use_case.clone();
    use_case.execute(id.to_string()).await
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<UpdateUserDto>,
) -> impl IntoResponse {
    let mut dto = dto;
    dto.id = id;
    let use_case = state.use_cases.update_user_use_case.clone();
    use_case.execute(dto).await
}

pub async fn delete_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let use_case = state.use_cases.delete_user_by_id_use_case.clone();
    use_case.execute(id.to_string()).await
}
