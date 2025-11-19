// Import Domain
use crate::entities::users::{self, Entity as Users, Model as UserModel};
use sea_orm::*;

// Import Dtos
use super::model::{
    AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
};

// Import Utils
use crate::domain::error::AppError;
use crate::utils::jwt::{JwtUtil, TokenType};
use crate::utils::password::{hash_password, verify_password};
use validator::Validate;

/// UserService handles authentication and user management logic
#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
    jwt_util: JwtUtil,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new(
        db: DatabaseConnection,
        jwt_secret: &str,
        access_token_expiration_hours: i64,
        refresh_token_expiration_hours: i64,
    ) -> Self {
        Self {
            db,
            jwt_util: JwtUtil::new(
                jwt_secret,
                access_token_expiration_hours,
                refresh_token_expiration_hours,
            ),
        }
    }

    /// Register a new user
    pub async fn register(&self, register_req: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Validate input
        register_req.validate().map_err(AppError::from)?;

        // Validate that at least username or email is provided
        if register_req.username.is_none() && register_req.email.is_none() {
            return Err(AppError::validation(
                "Either username or email must be provided",
            ));
        }

        // Check if username already exists
        if let Some(username) = &register_req.username {
            let existing_user = Users::find()
                .filter(users::Column::Username.eq(username))
                .one(&self.db)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            if existing_user.is_some() {
                return Err(AppError::username_already_exists(username));
            }
        }

        // Check if email already exists
        if let Some(email) = &register_req.email {
            let existing_user = Users::find()
                .filter(users::Column::Email.eq(email))
                .one(&self.db)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            if existing_user.is_some() {
                return Err(AppError::email_already_exists(email));
            }
        }

        // Hash password
        let password_hash =
            hash_password(&register_req.password).map_err(|e| AppError::Internal(e.to_string()))?;

        // Create user model
        let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
        let user_id = uuid::Uuid::now_v7();

        let active_model = users::ActiveModel {
            id: Set(user_id),
            username: Set(register_req.username.clone()),
            email: Set(register_req.email.clone()),
            display_name: Set(register_req.display_name.clone()),
            password_hash: Set(password_hash),
            role: Set("student".to_string()),
            status: Set("active".to_string()),
            created: Set(now),
            updated: Set(now),
        };

        let user_model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Generate tokens
        let email = user_model.email.as_deref().unwrap_or("");
        let access_token = self
            .jwt_util
            .generate_access_token(user_model.id, email)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let refresh_token = self
            .jwt_util
            .generate_refresh_token(user_model.id, email)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: user_model.into(),
        })
    }

    /// Login user
    pub async fn login(&self, login_req: LoginRequest) -> Result<AuthResponse, AppError> {
        // Validate input
        login_req.validate().map_err(AppError::from)?;

        // Find user by username or email
        let user_model = Users::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(&login_req.login))
                    .add(users::Column::Email.eq(&login_req.login)),
            )
            .one(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // Verify password
        if !verify_password(&login_req.password, &user_model.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Check user status
        if user_model.status != "active" {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        // Generate tokens
        let email = user_model.email.as_deref().unwrap_or("");
        let access_token = self
            .jwt_util
            .generate_access_token(user_model.id, email)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let refresh_token = self
            .jwt_util
            .generate_refresh_token(user_model.id, email)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: user_model.into(),
        })
    }

    /// Refresh access token
    pub async fn refresh_token(
        &self,
        refresh_req: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Validate input
        refresh_req.validate().map_err(AppError::from)?;

        // Validate refresh token
        let claims = self
            .jwt_util
            .verify_token(&refresh_req.token)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        if claims.token_type != TokenType::Refresh {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        let user_id = claims
            .sub
            .parse::<uuid::Uuid>()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        let user = Users::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        if user.status != "active" && user.status != "pending" {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        // Generate new access token
        let email = user.email.as_deref().unwrap_or("");
        let access_token = self
            .jwt_util
            .generate_access_token(user_id, email)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(RefreshTokenResponse { access_token })
    }

    /// Verify access token and return user
    pub async fn verify_token(&self, token: &str) -> Result<UserModel, AppError> {
        let claims = self
            .jwt_util
            .verify_token(token)
            .map_err(|_| AppError::Unauthorized("Invalid access token".to_string()))?;

        if claims.token_type != TokenType::Access {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        let user_id = claims
            .sub
            .parse::<uuid::Uuid>()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        let user_model = Users::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        if user_model.status != "active" && user_model.status != "pending" {
            return Err(AppError::Forbidden(
                "Account is suspended or inactive".to_string(),
            ));
        }

        Ok(user_model)
    }
}
