use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::application::use_cases::UseCase;
use crate::entities::{CreatePostDto, UpdatePostDto};
use crate::state::AppState;

pub async fn create_post(
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<CreatePostDto>,
) -> impl IntoResponse {
    let use_case = state.use_cases.add_post_use_case.clone();
    use_case.execute(dto).await
}

pub async fn get_posts(State(state): State<AppState>) -> impl IntoResponse {
    let use_case = state.use_cases.get_all_posts_use_case.clone();
    use_case.execute(()).await
}

pub async fn get_post_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let use_case = state.use_cases.get_post_by_id_use_case.clone();
    use_case.execute(id.to_string()).await
}

pub async fn update_post(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(dto): Json<UpdatePostDto>,
) -> impl IntoResponse {
    let mut dto = dto;
    dto.id = id;
    let use_case = state.use_cases.update_post_use_case.clone();
    use_case.execute(dto).await
}

pub async fn delete_post_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let use_case = state.use_cases.delete_post_by_id_use_case.clone();
    use_case.execute(id.to_string()).await
}
