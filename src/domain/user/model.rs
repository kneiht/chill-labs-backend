use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::users::Model as UserModel;

// Role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Student,
    Teacher,
    Admin,
}

// UserStatus enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Pending,
    Suspended,
}

// Validation error for User
#[derive(Debug, Clone)]
pub struct UserValidationError {
    pub message: String,
}

impl std::fmt::Display for UserValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UserValidationError {}

// ============= Auth Request DTOs =============

/// Request body for user registration
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: Option<String>,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

/// Request body for user login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    /// Can be either email or username
    #[validate(length(min = 1, message = "Login identifier cannot be empty"))]
    pub login: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

/// Request body for token refresh
#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Token cannot be empty"))]
    pub token: String,
}

// ============= Auth Response DTOs =============

/// Response for authentication operations (login, register)
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

/// User information returned in auth responses
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: UserStatus,
}

impl From<UserModel> for UserInfo {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id.to_string(),
            display_name: user.display_name.unwrap_or_default(),
            username: user.username.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            role: match user.role.as_str() {
                "admin" => Role::Admin,
                "teacher" => Role::Teacher,
                _ => Role::Student,
            },
            status: match user.status.as_str() {
                "active" => UserStatus::Active,
                "pending" => UserStatus::Pending,
                "suspended" => UserStatus::Suspended,
                _ => UserStatus::Suspended,
            },
        }
    }
}

/// Token refresh response
#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
}
