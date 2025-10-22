use crate::domain::response::{ErrorType, Response};
use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Custom error types for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    // Validation errors
    Validation(String),
    InvalidEmail(String),
    InvalidPassword(String),
    MissingField(String),
    UserValidationError(String),

    // Business logic errors
    NotFound(String),
    AlreadyExists(String),
    UsernameAlreadyExists(String),
    Conflict(String),

    // Authentication/Authorization errors
    Unauthorized(String),
    Forbidden(String),

    // Database errors
    DatabaseError(String),
    ConnectionError(String),

    // External service errors
    ExternalServiceError(String),

    // Internal errors
    Internal(String),
}

// Implement the Display trait for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidEmail(msg) => write!(f, "Invalid email: {}", msg),
            AppError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
            AppError::MissingField(msg) => write!(f, "Missing field: {}", msg),
            AppError::UserValidationError(msg) => write!(f, "User validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            AppError::UsernameAlreadyExists(msg) => write!(f, "Username already exists: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// Implement the Error trait for AppError
impl std::error::Error for AppError {}

// Implement the From trait for AnyhowError
impl From<AnyhowError> for AppError {
    fn from(err: AnyhowError) -> Self {
        AppError::Internal(err.to_string())
    }
}

// Implement the From trait for sqlx::Error
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => {
                // Check for specific database constraint violations
                if db_err.constraint().is_some() {
                    if db_err.message().contains("unique") || db_err.message().contains("duplicate")
                    {
                        if db_err.message().contains("username") {
                            AppError::UsernameAlreadyExists("Username already exists".to_string())
                        } else {
                            AppError::AlreadyExists("Resource already exists".to_string())
                        }
                    } else if db_err.message().contains("foreign key") {
                        AppError::Conflict("Referenced resource does not exist".to_string())
                    } else {
                        AppError::DatabaseError(db_err.message().to_string())
                    }
                } else {
                    AppError::DatabaseError(db_err.message().to_string())
                }
            }
            sqlx::Error::PoolTimedOut => {
                AppError::ConnectionError("Database connection timeout".to_string())
            }
            sqlx::Error::PoolClosed => {
                AppError::ConnectionError("Database pool is closed".to_string())
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// Implement the From trait for uuid::Error
impl From<uuid::Error> for AppError {
    fn from(_: uuid::Error) -> Self {
        AppError::Validation("Invalid UUID format".to_string())
    }
}

// Implement the From trait for serde_json::Error
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation(format!("JSON parsing error: {}", err))
    }
}

// Implement the From trait for UserValidationError
impl From<crate::domain::user::model::UserValidationError> for AppError {
    fn from(err: crate::domain::user::model::UserValidationError) -> Self {
        AppError::UserValidationError(err.message)
    }
}

/// Extension trait to convert AppError to Response
pub trait ToResponse<T> {
    fn to_response(self, success_message: &str) -> Response<T>;
    fn to_response_created(self, success_message: &str) -> Response<T>;
    fn to_response_no_content(self, success_message: &str) -> Response<serde_json::Value>;
}

// Implement the ToResponse trait for Result<T, AppError>
impl<T> ToResponse<T> for Result<T, AppError> {
    fn to_response(self, success_message: &str) -> Response<T> {
        match self {
            Ok(data) => Response::success_ok(data, success_message),
            Err(err) => {
                let (error_type, message, error_detail) = match err {
                    AppError::Validation(msg) => {
                        (ErrorType::Validation, "Validation failed", Some(msg))
                    }
                    AppError::InvalidEmail(msg) => {
                        (ErrorType::Validation, "Invalid email format", Some(msg))
                    }
                    AppError::InvalidPassword(msg) => {
                        (ErrorType::Validation, "Invalid password", Some(msg))
                    }
                    AppError::MissingField(msg) => {
                        (ErrorType::Validation, "Missing required field", Some(msg))
                    }
                    AppError::UserValidationError(msg) => {
                        (ErrorType::Validation, "User validation failed", Some(msg))
                    }
                    AppError::NotFound(msg) => {
                        (ErrorType::NotFound, "Resource not found", Some(msg))
                    }
                    AppError::AlreadyExists(msg) => {
                        (ErrorType::Conflict, "Resource already exists", Some(msg))
                    }
                    AppError::UsernameAlreadyExists(msg) => {
                        (ErrorType::Conflict, "Username already exists", Some(msg))
                    }
                    AppError::Conflict(msg) => {
                        (ErrorType::Conflict, "Resource conflict", Some(msg))
                    }
                    AppError::Unauthorized(msg) => {
                        (ErrorType::Unauthorized, "Unauthorized access", Some(msg))
                    }
                    AppError::Forbidden(msg) => {
                        (ErrorType::Forbidden, "Access forbidden", Some(msg))
                    }
                    AppError::DatabaseError(msg) => {
                        (ErrorType::Internal, "Database operation failed", Some(msg))
                    }
                    AppError::ConnectionError(msg) => {
                        (ErrorType::Internal, "Connection failed", Some(msg))
                    }
                    AppError::ExternalServiceError(msg) => {
                        (ErrorType::Internal, "External service error", Some(msg))
                    }
                    AppError::Internal(msg) => {
                        (ErrorType::Internal, "Internal server error", Some(msg))
                    }
                };

                Response::failure(message, error_type, error_detail)
            }
        }
    }

    fn to_response_created(self, success_message: &str) -> Response<T> {
        match self {
            Ok(data) => Response::success_created(data, success_message),
            Err(err) => {
                let (error_type, message, error_detail) = match err {
                    AppError::Validation(msg) => {
                        (ErrorType::Validation, "Validation failed", Some(msg))
                    }
                    AppError::InvalidEmail(msg) => {
                        (ErrorType::Validation, "Invalid email format", Some(msg))
                    }
                    AppError::InvalidPassword(msg) => {
                        (ErrorType::Validation, "Invalid password", Some(msg))
                    }
                    AppError::MissingField(msg) => {
                        (ErrorType::Validation, "Missing required field", Some(msg))
                    }
                    AppError::UserValidationError(msg) => {
                        (ErrorType::Validation, "User validation failed", Some(msg))
                    }
                    AppError::NotFound(msg) => {
                        (ErrorType::NotFound, "Resource not found", Some(msg))
                    }
                    AppError::AlreadyExists(msg) => {
                        (ErrorType::Conflict, "Resource already exists", Some(msg))
                    }
                    AppError::UsernameAlreadyExists(msg) => {
                        (ErrorType::Conflict, "Username already exists", Some(msg))
                    }
                    AppError::Conflict(msg) => {
                        (ErrorType::Conflict, "Resource conflict", Some(msg))
                    }
                    AppError::Unauthorized(msg) => {
                        (ErrorType::Unauthorized, "Unauthorized access", Some(msg))
                    }
                    AppError::Forbidden(msg) => {
                        (ErrorType::Forbidden, "Access forbidden", Some(msg))
                    }
                    AppError::DatabaseError(msg) => {
                        (ErrorType::Internal, "Database operation failed", Some(msg))
                    }
                    AppError::ConnectionError(msg) => {
                        (ErrorType::Internal, "Connection failed", Some(msg))
                    }
                    AppError::ExternalServiceError(msg) => {
                        (ErrorType::Internal, "External service error", Some(msg))
                    }
                    AppError::Internal(msg) => {
                        (ErrorType::Internal, "Internal server error", Some(msg))
                    }
                };

                Response::failure(message, error_type, error_detail)
            }
        }
    }

    fn to_response_no_content(self, success_message: &str) -> Response<serde_json::Value> {
        match self {
            Ok(_) => Response::success_no_content(success_message),
            Err(err) => {
                let (error_type, message, error_detail) = match err {
                    AppError::NotFound(msg) => {
                        (ErrorType::NotFound, "Resource not found", Some(msg))
                    }
                    AppError::DatabaseError(msg) => {
                        (ErrorType::Internal, "Database operation failed", Some(msg))
                    }
                    AppError::ConnectionError(msg) => {
                        (ErrorType::Internal, "Connection failed", Some(msg))
                    }
                    _ => (
                        ErrorType::Internal,
                        "Operation failed",
                        Some(err.to_string()),
                    ),
                };

                Response::failure(message, error_type, error_detail)
            }
        }
    }
}

/// Helper functions for common error scenarios
impl AppError {
    pub fn user_not_found(id: Uuid) -> Self {
        AppError::NotFound(format!("User with id {} not found", id))
    }

    pub fn email_already_exists(email: &str) -> Self {
        AppError::AlreadyExists(format!("Email {} already exists", email))
    }

    pub fn username_already_exists(username: &str) -> Self {
        AppError::UsernameAlreadyExists(format!("Username {} already exists", username))
    }

    pub fn invalid_password() -> Self {
        AppError::InvalidPassword("Password does not meet requirements".to_string())
    }

    pub fn invalid_email_format(email: &str) -> Self {
        AppError::InvalidEmail(format!("Invalid email format: {}", email))
    }

    pub fn missing_field(field_name: &str) -> Self {
        AppError::MissingField(format!("Field '{}' is required", field_name))
    }

    pub fn unauthorized(message: &str) -> Self {
        AppError::Unauthorized(message.to_string())
    }

    pub fn forbidden(message: &str) -> Self {
        AppError::Forbidden(message.to_string())
    }

    pub fn validation(message: &str) -> Self {
        AppError::Validation(message.to_string())
    }
}
