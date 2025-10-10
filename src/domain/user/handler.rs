use super::model::{Role, User, UserStatus};
use crate::domain::response::UseCaseResponse;
use crate::state::AppState;
use crate::utils::password::hash_password;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub status: Option<UserStatus>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub display_name: String,
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
) -> UseCaseResponse<UserResponse> {
    let user_service = state.user_service.clone();

    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return UseCaseResponse::failure_internal(
                "Failed to hash password",
                Some(e.to_string()),
            )
        }
    };

    match user_service
        .create_user(req.display_name, req.email, password_hash, req.role)
        .await
    {
        Ok(user) => UseCaseResponse::success_created(user.into(), "User created successfully"),
        Err(e) => UseCaseResponse::failure_internal("Failed to create user", Some(e.to_string())),
    }
}

pub async fn get_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> UseCaseResponse<UserResponse> {
    let user_service = state.user_service.clone();

    match user_service.get_user_by_id(id).await {
        Ok(user) => UseCaseResponse::success_ok(user.into(), "User retrieved successfully"),
        Err(e) => UseCaseResponse::failure_not_found("User not found", Some(e.to_string())),
    }
}

pub async fn get_all_users(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> UseCaseResponse<Vec<UserResponse>> {
    let user_service = state.user_service.clone();

    match user_service.get_all_users().await {
        Ok(users) => UseCaseResponse::success_ok(
            users.into_iter().map(Into::into).collect(),
            "Users retrieved successfully",
        ),
        Err(e) => {
            UseCaseResponse::failure_internal("Failed to retrieve users", Some(e.to_string()))
        }
    }
}

pub async fn update_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> UseCaseResponse<UserResponse> {
    let user_service = state.user_service.clone();

    match user_service
        .update_user(id, req.display_name, req.email, req.role, req.status)
        .await
    {
        Ok(user) => UseCaseResponse::success_ok(user.into(), "User updated successfully"),
        Err(e) => UseCaseResponse::failure_internal("Failed to update user", Some(e.to_string())),
    }
}

pub async fn delete_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> UseCaseResponse<serde_json::Value> {
    let user_service = state.user_service.clone();

    match user_service.delete_user(id).await {
        Ok(()) => UseCaseResponse::success_no_content("User deleted successfully"),
        Err(e) => UseCaseResponse::failure_internal("Failed to delete user", Some(e.to_string())),
    }
}
