use crate::middleware::global::RequestId;
use crate::utils::errors::{client_error_response, server_error_response};
use axum::http::StatusCode;
use axum::response::IntoResponse;

// =============================================================================
// =                         Service Error Handling                            =
// =============================================================================
#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("User with email {email} already exists")]
    UserAlreadyExists { email: String },

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Current password is incorrect")]
    InvalidCurrentPassword,

    #[error("Email is already verified")]
    EmailAlreadyVerified,

    #[error("Email verification token is empty")]
    EmailVerificationTokenEmpty,

    #[error("Invalid or expired verification token")]
    InvalidVerificationToken,

    #[error("Verification token has expired")]
    VerificationTokenExpired,

    #[error("Failed to send verification email: {email}")]
    EmailSendError { email: String },

    #[error("Database error: {source}")]
    RepositoryError { source: anyhow::Error },

    #[error("Internal server error: {source}")]
    InternalError { source: anyhow::Error },
}

impl UserServiceError {
    pub fn to_user_error(self, request_id: &RequestId) -> UserError {
        UserError {
            request_id: request_id.clone(),
            source: self,
        }
    }
}

// =============================================================================
// =                       Handler Error Handling                              =
// =============================================================================
#[derive(Debug, thiserror::Error)]
#[error("UserError: {source}")]
pub struct UserError {
    pub request_id: RequestId,
    pub source: UserServiceError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        match self.source {
            UserServiceError::UserAlreadyExists { email } => client_error_response(
                StatusCode::CONFLICT,
                format!("User with email {email} already exists"),
                self.request_id,
            )
            .into_response(),
            UserServiceError::UserNotFound => {
                client_error_response(StatusCode::NOT_FOUND, "User not found", self.request_id)
                    .into_response()
            }
            UserServiceError::InvalidCredentials => client_error_response(
                StatusCode::UNAUTHORIZED,
                "Invalid email or password",
                self.request_id,
            )
            .into_response(),
            UserServiceError::InvalidCurrentPassword => client_error_response(
                StatusCode::UNAUTHORIZED,
                "Current password is incorrect",
                self.request_id,
            )
            .into_response(),
            UserServiceError::EmailAlreadyVerified => client_error_response(
                StatusCode::BAD_REQUEST,
                "Email is already verified",
                self.request_id,
            )
            .into_response(),
            UserServiceError::EmailVerificationTokenEmpty => client_error_response(
                StatusCode::BAD_REQUEST,
                "Email verification token is empty",
                self.request_id,
            )
            .into_response(),
            UserServiceError::InvalidVerificationToken => client_error_response(
                StatusCode::BAD_REQUEST,
                "Invalid or expired verification token",
                self.request_id,
            )
            .into_response(),
            UserServiceError::VerificationTokenExpired => client_error_response(
                StatusCode::BAD_REQUEST,
                "Verification token has expired",
                self.request_id,
            )
            .into_response(),

            UserServiceError::EmailSendError { email } => client_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send verification email: {email}"),
                self.request_id,
            )
            .into_response(),
            UserServiceError::RepositoryError { source } => {
                tracing::error!(
                    "Database error: {}, request_id: {}",
                    source,
                    self.request_id
                );
                client_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error",
                    self.request_id,
                )
                .into_response()
            }
            UserServiceError::InternalError { source } => {
                tracing::error!(
                    "Internal Server Error (original): {}, request_id: {}",
                    source,
                    self.request_id,
                );

                server_error_response(
                    "An unexpected error occurred on the server. Please try again later.",
                    self.request_id,
                )
                .into_response()
            }
        }
    }
}
