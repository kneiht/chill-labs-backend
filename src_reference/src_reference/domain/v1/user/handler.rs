use super::error::{UserError, UserServiceError};
use super::model::{Gender, Membership, User, UserRole, UserStatus};
use super::service::{
    AuthUserInput, ChangePasswordInput, CreateUserServiceInput, UpdateProfileInput,
};
use crate::middleware::global::RequestId;
use crate::settings::ServerEnv;
use crate::state::AppState;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;
use axum::{
    extract::{Json, Query, State},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Extension,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Define the DTO LoginRequest struct
#[derive(Debug, Deserialize, Validate)] // Derive Validate
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    // Basic presence validation for password. Consider more complex rules if needed.
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

// Define the DTO SignupRequest struct
#[derive(Debug, Deserialize, Validate)] // Derive Validate
pub struct SignupRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    // You might want to add regex for password complexity here too
    pub password: String,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Display name must be between 1 and 50 characters"
    ))]
    pub display_name: String,
}

// Define the DTO ChangePasswordRequest struct
#[derive(Debug, Deserialize, Validate)] // Derive Validate
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password cannot be empty"))]
    pub current_password: String,
    #[validate(length(min = 8, message = "New password must be at least 8 characters long"))]
    pub new_password: String,
}

// Define the DTO UpdateProfileRequest struct
#[derive(Debug, Deserialize, Validate)] // Derive Validate
pub struct UpdateProfileRequest {
    // Validator handles Option fields: it validates if Some, skips if None.
    #[validate(length(
        min = 1,
        max = 50,
        message = "Display name must be between 1 and 50 characters"
    ))]
    pub display_name: Option<String>,

    // #[validate(custom(function = "validate_optional_url"))]
    // TODO: Add custom validator for URL
    pub avatar_url: Option<Option<String>>,

    pub status: Option<UserStatus>,
    pub role: Option<UserRole>,

    // Using Option<Option<T>> to distinguish between "not provided" and "provided as null"
    pub membership: Option<Membership>,
    pub gender: Option<Gender>,

    pub date_of_birth: Option<Option<NaiveDate>>,

    #[validate(length(min = 7, max = 20, message = "Phone number is invalid"))]
    // Basic length validation
    pub phone: Option<Option<String>>,

    #[validate(length(max = 500, message = "Bio cannot exceed 500 characters"))]
    pub bio: Option<Option<String>>,
    // TODO: Consider more specific validation for phone (e.g., regex)
    // TODO: For date_of_birth, ensure it's a valid date format if string, or use a date type.
}
// Define the DTO UserResponse struct
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub status: UserStatus,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub created: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    // New fields for response
    pub membership: Membership,
    pub gender: Gender,
    pub date_of_birth: Option<NaiveDate>,
    pub phone: Option<String>,
    pub bio: Option<String>,
}

// Define the DTO VerifyEmailParams struct
#[derive(Debug, Deserialize)]
pub struct VerifyEmailParams {
    pub token: String,
}

// Login handler
pub async fn login(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(login_req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the request body
    login_req
        .validate()
        .map_err(|e| AppError::validation("Invalid login request", e, &request_id))?;

    // Get the user service
    let user_service = &state.services_v1.user_service;
    let jwt_util = &state.jwt_util;

    // Prepare input for the service layer
    let service_input = AuthUserInput {
        email: login_req.email,
        password: login_req.password,
    };

    // Authenticate the user
    let user = user_service
        .authenticate_user(service_input)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    // Generate a JWT token
    let token = jwt_util.generate_token(user.id, &user.email).map_err(|e| {
        AppError::internal("Failed to generate authentication token", e, &request_id)
    })?;

    // Determine cookie max-age from JWT expiration settings
    let jwt_expiration_hours = state.settings.load().jwt.expiration_hours.unwrap_or(24);
    let max_age_seconds = jwt_expiration_hours * 60 * 60;

    // Conditionally add Secure attribute to cookie
    let mut cookie_str = format!(
        "auth_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
        token, max_age_seconds
    );

    // Add Secure attribute unless in Dev environment
    let is_dev_env = match state.settings.load().server.env.as_ref() {
        Some(ServerEnv::Dev) => true,
        _ => false,
    };

    if !is_dev_env {
        cookie_str.push_str("; Secure");
    }

    // Create the response
    Ok(ApiResponse::<UserResponse, ()>::success(
        StatusCode::OK,
        "Login successful",
        user.into(),
        request_id,
        Some(AppendHeaders([(header::SET_COOKIE, cookie_str)].to_vec())),
    ))
}

// Signup handler
pub async fn signup(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(signup_req): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the request body
    signup_req
        .validate()
        .map_err(|e| AppError::validation("Invalid signup request", e, &request_id))?;

    // Get the user service
    let user_service = &state.services_v1.user_service;
    let jwt_util = &state.jwt_util;

    // Prepare input for the service layer
    let service_input = CreateUserServiceInput {
        email: signup_req.email,
        password: signup_req.password,
        display_name: signup_req.display_name,
    };

    // Create the user
    let user = user_service
        .create_user(service_input)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    // Send verification email (non-blocking)
    if let Err(e) = user_service.send_verification_email(user.id).await {
        tracing::error!("Failed to send verification email: {}", e);
    }

    // Generate a JWT token
    let token = jwt_util.generate_token(user.id, &user.email).map_err(|e| {
        AppError::internal("Failed to generate authentication token", e, &request_id)
    })?;

    // Determine cookie max-age from JWT expiration settings
    let jwt_expiration_hours = state.settings.load().jwt.expiration_hours.unwrap_or(24);
    let max_age_seconds = jwt_expiration_hours * 60 * 60;

    // Conditionally add Secure attribute to cookie
    let mut cookie_str = format!(
        "auth_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
        token, max_age_seconds
    );

    // Add Secure attribute unless in Dev environment
    let is_dev_env = match state.settings.load().server.env.as_ref() {
        Some(ServerEnv::Dev) => true,
        _ => false,
    };
    if !is_dev_env {
        cookie_str.push_str("; Secure");
    }

    Ok(ApiResponse::<UserResponse, ()>::success(
        StatusCode::CREATED,
        "User created successfully",
        user.into(),
        request_id,
        Some(AppendHeaders([(header::SET_COOKIE, cookie_str)].to_vec())),
    ))
}

// Change password handler
pub async fn change_password(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Extension(request_id): Extension<RequestId>,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the request body
    req.validate()
        .map_err(|e| AppError::validation("Invalid password change request", e, &request_id))?;

    // Get the user service
    let user_service = &state.services_v1.user_service;

    // Prepare input for the service layer
    let service_input = ChangePasswordInput {
        user_id, // From Extension
        current_password: req.current_password,
        new_password: req.new_password,
    };

    // Change the password
    user_service
        .change_password(service_input)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    Ok(ApiResponse::<(), ()>::success(
        StatusCode::OK,
        "Password changed successfully",
        (),
        request_id,
        None,
    ))
}

// Get current user handler
pub async fn get_current_user(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Extension(request_id): Extension<RequestId>,
) -> Result<impl IntoResponse, AppError> {
    // Get the user service
    let user_service = &state.services_v1.user_service;

    // Get the user
    let user = user_service
        .get_user_by_id(user_id)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    // Create the response
    Ok(ApiResponse::<UserResponse, ()>::success(
        StatusCode::OK,
        "User retrieved successfully",
        user.into(),
        request_id,
        None,
    ))
}

// Update profile handler
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Extension(request_id): Extension<RequestId>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the request body
    req.validate()
        .map_err(|e| AppError::validation("Invalid profile update request", e, &request_id))?;

    let user_service = &state.services_v1.user_service;

    // Prepare input for the service layer
    let service_input = UpdateProfileInput {
        user_id, // From Extension
        display_name: req.display_name,
        avatar_url: req.avatar_url,
        status: req.status,
        role: req.role,
        membership: req.membership,
        gender: req.gender,
        date_of_birth: req.date_of_birth,
        phone: req.phone,
        bio: req.bio,
    };

    // Call the service method
    let updated_user = user_service
        .update_profile(service_input)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    // Create the response
    Ok(ApiResponse::<UserResponse, ()>::success(
        StatusCode::OK,
        "Profile updated successfully",
        updated_user.into(),
        request_id,
        None,
    ))
}

// Handler for initiating email verification (resending verification email)
pub async fn resend_verification_email(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Extension(request_id): Extension<RequestId>,
) -> Result<impl IntoResponse, AppError> {
    // Get the user service
    let user_service = &state.services_v1.user_service;

    // Get the user
    let user = user_service
        .get_user_by_id(user_id)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    // Check if the email is already verified
    if user.email_verified {
        return Err(UserError {
            request_id: request_id.clone(),
            source: UserServiceError::EmailAlreadyVerified,
        }
        .into());
    }

    // Send verification email
    user_service
        .send_verification_email(user_id)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    Ok(ApiResponse::<(), ()>::success(
        StatusCode::OK,
        "Verification email resent successfully",
        (),
        request_id,
        None,
    ))
}

// Handler for confirming email verification (receiving a token)
pub async fn verify_email(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Query(params): Query<VerifyEmailParams>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the token
    if params.token.is_empty() {
        return Err(UserError {
            request_id: request_id.clone(),
            source: UserServiceError::EmailVerificationTokenEmpty,
        }
        .into());
    }

    // Get the user service
    let user_service = &state.services_v1.user_service;

    // Verify the email
    user_service
        .verify_email_with_token(&params.token)
        .await
        .map_err(|e| e.to_user_error(&request_id))?;

    Ok(ApiResponse::<(), ()>::success(
        StatusCode::OK,
        "Email verified successfully",
        (),
        request_id,
        None,
    ))
}

// --- Conversion Implementation ---

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            display_name: user.display_name, // Wrap in Some to match UserResponse field type
            email: user.email,               // Ensure User model has email
            email_verified: user.email_verified,
            status: user.status,
            role: user.role,
            avatar_url: user.avatar_url,
            created: user.created,
            last_login: user.last_login,
            // Map new fields
            membership: user.membership,
            gender: user.gender,
            date_of_birth: user.date_of_birth,
            phone: user.phone,
            bio: user.bio,
        }
    }
}

// Custom validation function for Option<Option<String>> URL
// fn validate_optional_url(
//     opt_opt_url: &Option<Option<String>>,
// ) -> Result<(), validator::ValidationError> {
//     if let Some(Some(url_str)) = opt_opt_url {
//         if url_str.validate_url() {
//             return Ok(());
//         }
//         return Err(validator::ValidationError::new("invalid_url_format"));
//     }
//     Ok(()) // It's valid if it's None or Some(None)
// }
