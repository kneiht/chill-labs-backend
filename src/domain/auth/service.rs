// Import Domain
use crate::domain::user::model::{Role, User};
use crate::domain::user::repository::UserRepository;
use crate::domain::user::service::{CreateUserInput, UserService};

// Import Dtos
use super::model::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest};

// Import Utils
use crate::domain::error::AppError;
use crate::domain::Transformer;
use crate::utils::jwt::{Claims, JwtUtil};
use crate::utils::password::{hash_password, verify_password};

/// AuthService handles authentication and authorization logic
#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    jwt_util: JwtUtil,
}

impl AuthService {
    /// Create a new AuthService instance
    pub fn new(
        user_repository: UserRepository,
        jwt_secret: &str,
        jwt_expiration_hours: i64,
    ) -> Self {
        Self {
            user_service: UserService::new(user_repository),
            jwt_util: JwtUtil::new(jwt_secret, jwt_expiration_hours),
        }
    }

    /// Register a new user
    pub async fn register<T: Transformer<RegisterRequest>>(
        &self,
        to_register_request: T,
    ) -> Result<AuthResponse, AppError> {
        // Validate and transform input
        let register_req = to_register_request.transform()?;

        // Validate that at least username or email is provided
        if register_req.username.is_none() && register_req.email.is_none() {
            return Err(AppError::validation(
                "Either username or email must be provided",
            ));
        }

        // Hash the password
        let password_hash = hash_password(&register_req.password)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

        // Create user input
        let create_input = CreateUserInput {
            display_name: register_req.display_name,
            username: register_req.username,
            email: register_req.email,
            password_hash,
            role: Role::Student, // Default role for new registrations
        };

        // Create user through user service
        let user = self.user_service.create_user(create_input).await?;

        // Generate JWT token
        let empty_email = String::new();
        let email = user.email.as_ref().unwrap_or(&empty_email);
        let token = self
            .jwt_util
            .generate_token(user.id, email)
            .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    /// Login user with email or username and password
    pub async fn login<T: Transformer<LoginRequest>>(
        &self,
        to_login_request: T,
    ) -> Result<AuthResponse, AppError> {
        // Validate and transform input
        let login_req = to_login_request.transform()?;

        // Try to find user by email first, then by username
        let user = if login_req.login.contains('@') {
            // Looks like an email
            self.user_service
                .get_user_by_email(&login_req.login)
                .await?
        } else {
            // Treat as username
            self.user_service
                .get_user_by_username(&login_req.login)
                .await?
        };

        // Verify password
        let is_valid = verify_password(&login_req.password, &user.password_hash)
            .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Check if user is active
        if user.status != crate::domain::user::model::UserStatus::Active
            && user.status != crate::domain::user::model::UserStatus::Pending
        {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        // Generate JWT token
        let empty_email = String::new();
        let email = user.email.as_ref().unwrap_or(&empty_email);
        let token = self
            .jwt_util
            .generate_token(user.id, email)
            .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    /// Refresh an existing JWT token
    pub async fn refresh_token<T: Transformer<RefreshTokenRequest>>(
        &self,
        to_refresh_request: T,
    ) -> Result<String, AppError> {
        // Validate and transform input
        let refresh_req = to_refresh_request.transform()?;

        // Verify the existing token
        let claims = self
            .jwt_util
            .verify_token(&refresh_req.token)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        // Parse user ID from claims
        let user_id = claims
            .sub
            .parse()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        // Verify user still exists and is active
        let user = self.user_service.get_user_by_id(user_id).await?;

        if user.status != crate::domain::user::model::UserStatus::Active
            && user.status != crate::domain::user::model::UserStatus::Pending
        {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        // Generate new token
        let empty_email = String::new();
        let email = user.email.as_ref().unwrap_or(&empty_email);
        let new_token = self
            .jwt_util
            .generate_token(user.id, email)
            .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        Ok(new_token)
    }

    /// Verify token and return user information
    pub async fn verify_and_get_user(&self, token: &str) -> Result<User, AppError> {
        // Verify token
        let claims = self
            .jwt_util
            .verify_token(token)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        // Parse user ID from claims
        let user_id = claims
            .sub
            .parse()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        // Get user from database
        let user = self.user_service.get_user_by_id(user_id).await?;

        // Check if user is active
        if user.status != crate::domain::user::model::UserStatus::Active
            && user.status != crate::domain::user::model::UserStatus::Pending
        {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        Ok(user)
    }

    /// Verify token and return claims
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        self.jwt_util
            .verify_token(token)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }
}
