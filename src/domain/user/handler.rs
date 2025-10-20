use super::model::{Role, User, UserStatus};
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::state::AppState;
use crate::utils::password::hash_password;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub status: Option<UserStatus>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: UserStatus,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            display_name: user.display_name,
            username: user.username,
            email: user.email,
            role: user.role,
            status: user.status,
            created: user.created,
            updated: user.updated,
        }
    }
}

pub async fn create_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Response<UserResponse> {
    let user_service = state.user_service.clone();

    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return Response::failure_internal(
                "Failed to hash password",
                Some(format!("Password hashing failed: {}", e)),
            );
        }
    };

    user_service
        .create_user(req.display_name, req.username, req.email, password_hash, Role::Student)
        .await
        .map(|user| user.into())
        .to_response_created("User created successfully")
}

pub async fn get_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Response<UserResponse> {
    let user_service = state.user_service.clone();

    user_service
        .get_user_by_id(id)
        .await
        .map(|user| user.into())
        .to_response("User retrieved successfully")
}

pub async fn get_all_users(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Response<Vec<UserResponse>> {
    let user_service = state.user_service.clone();

    user_service
        .get_all_users()
        .await
        .map(|users| users.into_iter().map(Into::into).collect())
        .to_response("Users retrieved successfully")
}

pub async fn update_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Response<UserResponse> {
    let user_service = state.user_service.clone();

    user_service
        .update_user(id, req.display_name, req.username, req.email, req.role, req.status)
        .await
        .map(|user| user.into())
        .to_response("User updated successfully")
}

pub async fn delete_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Response<serde_json::Value> {
    let user_service = state.user_service.clone();

    user_service
        .delete_user(id)
        .await
        .to_response_no_content("User deleted successfully")
}
