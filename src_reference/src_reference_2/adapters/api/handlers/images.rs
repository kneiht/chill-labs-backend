use axum::{extract::State, response::IntoResponse};

use crate::application::use_cases::UseCase;
use crate::entities::CreateImageDto;
use crate::state::AppState;

pub async fn create_image(
    State(state): State<AppState>,
    axum::Json(dto): axum::Json<CreateImageDto>,
) -> impl IntoResponse {
    let use_case = state.use_cases.add_image_use_case.clone();
    use_case.execute(dto).await
}

pub async fn get_images(State(state): State<AppState>) -> impl IntoResponse {
    let use_case = state.use_cases.get_all_images_use_case.clone();
    use_case.execute(()).await
}
