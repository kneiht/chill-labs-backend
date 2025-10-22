use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

// ErrorType enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    Validation,
    NotFound,
    Unauthorized,
    Forbidden,
    Internal,
    Conflict,
}

// SuccessType enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessType {
    Ok,
    Created,
    NoContent,
}

// Status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Status {
    Success(SuccessType),
    Error(ErrorType),
}

// Pagination struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub pages: u32,
}

// Response struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> Response<T> {
    pub fn success(data: T, message: &str, status: SuccessType) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status: Status::Success(status),
            data: Some(data),
            pagination: None,
            error: None,
        }
    }

    pub fn success_ok(data: T, message: &str) -> Self {
        Self::success(data, message, SuccessType::Ok)
    }

    pub fn success_created(data: T, message: &str) -> Self {
        Self::success(data, message, SuccessType::Created)
    }

    pub fn success_no_content(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status: Status::Success(SuccessType::NoContent),
            data: None,
            pagination: None,
            error: None,
        }
    }

    pub fn failure(message: &str, error_type: ErrorType, error: Option<String>) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status: Status::Error(error_type),
            data: None,
            pagination: None,
            error,
        }
    }

    pub fn failure_validation(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::Validation, error)
    }

    pub fn failure_not_found(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::NotFound, error)
    }

    pub fn failure_unauthorized(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::Unauthorized, error)
    }

    pub fn failure_forbidden(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::Forbidden, error)
    }

    pub fn failure_internal(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::Internal, error)
    }

    pub fn failure_conflict(message: &str, error: Option<String>) -> Self {
        Self::failure(message, ErrorType::Conflict, error)
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            match self.status {
                Status::Error(ErrorType::Validation) => StatusCode::BAD_REQUEST,
                Status::Error(ErrorType::Unauthorized) => StatusCode::UNAUTHORIZED,
                Status::Error(ErrorType::Forbidden) => StatusCode::FORBIDDEN,
                Status::Error(ErrorType::NotFound) => StatusCode::NOT_FOUND,
                Status::Error(ErrorType::Conflict) => StatusCode::CONFLICT,
                Status::Error(ErrorType::Internal) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };

        (status_code, Json(self)).into_response()
    }
}
