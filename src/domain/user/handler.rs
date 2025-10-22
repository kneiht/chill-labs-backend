use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{Role, User, UserStatus};
use super::service::{CreateUserInput, UpdateUserInput};
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::state::AppState;

use crate::utils::password::hash_password;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
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
            display_name: user.display_name.unwrap_or_default(),
            username: user.username.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            role: user.role,
            status: user.status,
            created: user.created,
            updated: user.updated,
        }
    }
}

// Create user handler
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Response<UserResponse> {
    // Clone user service
    let user_service = state.user_service.clone();

    // Hash password
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return Response::failure_internal(
                "Failed to hash password",
                Some(format!("Password hashing failed: {}", e)),
            );
        }
    };

    // Create user input
    let create_input = CreateUserInput {
        display_name: req.display_name,
        username: req.username,
        email: req.email,
        password_hash,
        role: Role::Student,
    };

    // Create user
    user_service
        .create_user(create_input)
        .await
        .map(|user| user.into())
        .to_response_created("User created successfully")
}

// Get user handler
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response<UserResponse> {
    // Clone user service
    let user_service = state.user_service.clone();

    // Get user by id
    user_service
        .get_user_by_id(id)
        .await
        .map(|user| user.into())
        .to_response("User retrieved successfully")
}

// Get all users handler
pub async fn get_all_users(State(state): State<AppState>) -> Response<Vec<UserResponse>> {
    // Clone user service
    let user_service = state.user_service.clone();

    // Get all userss
    user_service
        .get_all_users()
        .await
        .map(|users| users.into_iter().map(Into::into).collect())
        .to_response("Users retrieved successfully")
}

// Update user handler
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Response<UserResponse> {
    // Clone user service
    let user_service = state.user_service.clone();

    // Update user input
    let update_input = UpdateUserInput {
        id,
        display_name: req.display_name,
        username: req.username,
        email: req.email,
        role: req.role,
        status: req.status,
    };

    // Update user
    user_service
        .update_user(update_input)
        .await
        .map(|user| user.into())
        .to_response("User updated successfully")
}

// Delete user handler
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response<serde_json::Value> {
    // Clone user service
    let user_service = state.user_service.clone();

    // Delete user
    user_service
        .delete_user(id)
        .await
        .to_response_no_content("User deleted successfully")
}
