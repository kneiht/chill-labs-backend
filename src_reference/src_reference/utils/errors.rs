use crate::domain::v1;
use crate::middleware::global::RequestId;
use crate::utils::response::ApiResponse;
use axum::{http::StatusCode, response::IntoResponse};
use std::collections::HashMap;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{message}")]
    Validation {
        request_id: RequestId,
        message: String,
        errors: ValidationErrors,
    },
    #[error("{message}")]
    PathNotFound {
        request_id: RequestId,
        message: String,
    },
    #[error("{message}")]
    Unauthorized {
        request_id: RequestId,
        message: String,
    },
    #[error("{message}")]
    InternalServerError {
        request_id: RequestId,
        message: String,
        source: anyhow::Error,
    },
    #[error(transparent)]
    UserV1 {
        #[from]
        source: v1::user::error::UserError,
    },
}

impl AppError {
    pub fn validation(
        message: impl Into<String>,
        errors: ValidationErrors,
        request_id: &RequestId,
    ) -> Self {
        Self::Validation {
            request_id: request_id.clone(),
            message: message.into(),
            errors,
        }
    }

    pub fn path_not_found(message: impl Into<String>, request_id: &RequestId) -> Self {
        Self::PathNotFound {
            request_id: request_id.clone(),
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>, request_id: &RequestId) -> Self {
        Self::Unauthorized {
            request_id: request_id.clone(),
            message: message.into(),
        }
    }

    pub fn internal(
        message: impl Into<String>,
        source: anyhow::Error,
        request_id: &RequestId,
    ) -> Self {
        Self::InternalServerError {
            request_id: request_id.clone(),
            message: message.into(),
            source,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Validation {
                request_id,
                message,
                errors,
            } => validation_error_response(message, errors, request_id).into_response(),
            AppError::PathNotFound {
                request_id,
                message,
            } => client_error_response(StatusCode::NOT_FOUND, message, request_id).into_response(),
            AppError::Unauthorized {
                request_id,
                message,
            } => {
                client_error_response(StatusCode::UNAUTHORIZED, message, request_id).into_response()
            }
            AppError::InternalServerError {
                request_id,
                message,
                source,
            } => {
                tracing::error!(
                    "Internal Server Error (original): {}, request_id: {}, source: {}",
                    message,
                    request_id,
                    source
                );
                server_error_response(
                    "An unexpected error occurred on the server. Please try again later.",
                    request_id,
                )
                .into_response()
            }
            AppError::UserV1 { source } => source.into_response(),
        }
    }
}

/// Creates a response for validation errors (HTTP 422).
pub fn validation_error_response(
    message: impl Into<String>,
    errors: ValidationErrors,
    request_id: RequestId,
) -> impl IntoResponse {
    ApiResponse::<(), ValidationErrors>::error(
        StatusCode::UNPROCESSABLE_ENTITY,
        message,
        errors,
        request_id,
    )
}

/// Creates a generic client error response (e.g., 400, 401, 403, 404).
pub fn client_error_response(
    status_code: StatusCode,
    message: impl Into<String>,
    request_id: RequestId,
) -> impl IntoResponse {
    let mut error_detail = HashMap::new();
    let owned_message = message.into();
    error_detail.insert("general".to_string(), vec![owned_message.clone()]);
    ApiResponse::<(), HashMap<String, Vec<_>>>::error(
        status_code,
        owned_message,
        error_detail,
        request_id,
    )
}

/// Creates a server error response (HTTP 500).
pub fn server_error_response(
    message: impl Into<String>,
    request_id: RequestId,
) -> impl IntoResponse {
    let mut error_detail = HashMap::new();
    let owned_message = message.into();
    error_detail.insert("internal".to_string(), vec![owned_message.clone()]);
    ApiResponse::<(), HashMap<String, Vec<_>>>::error(
        StatusCode::INTERNAL_SERVER_ERROR,
        owned_message,
        error_detail,
        request_id,
    )
}
