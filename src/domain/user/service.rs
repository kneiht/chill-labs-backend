use uuid::Uuid;
use validator::Validate;

// Import user model
use super::model::{Role, User, UserStatus};

// Import user repository
use super::repository::UserRepository;

// Import error handling
use crate::domain::error::AppError;

// Import Transformer trait
use crate::domain::Transformer;

// Create user dto
#[derive(Debug, Validate)]
pub struct CreateUserInput {
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: Option<String>,
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub password_hash: String,
    pub role: Role,
}

// Update user dto
#[derive(Debug, Validate)]
pub struct UpdateUserInput {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: Option<String>,
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub role: Option<Role>,
    pub status: Option<UserStatus>,
}

// UserService struct
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

// Implementation of UserService
impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    // Create user
    pub async fn create_user<T: Transformer<CreateUserInput>>(
        &self,
        to_create_user_input: T,
    ) -> Result<User, AppError> {
        // Validate and transform input
        let create_user_input = to_create_user_input.transform()?;

        // Create user
        let user = User::new(
            create_user_input.display_name,
            create_user_input.username,
            create_user_input.email,
            create_user_input.password_hash,
            create_user_input.role,
        );

        // Check if username already exists
        if let Some(username) = &user.username {
            if self
                .repository
                .find_by_username(username.as_str())
                .await?
                .is_some()
            {
                return Err(AppError::username_already_exists(username));
            }
        }

        // Check if email already exists if provided
        if let Some(email) = &user.email {
            if self
                .repository
                .find_by_email(email.as_str())
                .await?
                .is_some()
            {
                return Err(AppError::email_already_exists(email));
            }
        }

        self.repository.create(user).await
    }

    // Get user by id
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::user_not_found(id))
    }

    // Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<User, AppError> {
        self.repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))
    }

    // Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<User, AppError> {
        self.repository
            .find_by_username(username)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with username {} not found", username)))
    }

    // Get all users
    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all().await
    }

    // Update user
    pub async fn update_user<T: Transformer<UpdateUserInput>>(
        &self,
        to_update_user_input: T,
    ) -> Result<User, AppError> {
        // Validate and transform input
        let update_input = to_update_user_input.transform()?;

        // Get user by id
        let mut user = self.get_user_by_id(update_input.id).await?;

        // Update user fields
        if let Some(display_name) = update_input.display_name {
            user.display_name = Some(display_name);
        }

        if let Some(username) = update_input.username {
            // Check if new username conflicts
            if let Some(existing) = self.repository.find_by_username(&username[..]).await? {
                if existing.id != update_input.id {
                    return Err(AppError::username_already_exists(&username));
                }
            }
            user.username = Some(username);
        }

        if let Some(email) = update_input.email {
            if !email.trim().is_empty() {
                // Check if new email conflicts
                if let Some(existing) = self.repository.find_by_email(email.as_str()).await? {
                    if existing.id != update_input.id {
                        return Err(AppError::email_already_exists(&email));
                    }
                }
                user.email = Some(email);
            } else {
                // If email is provided as empty string, set it to empty
                user.email = Some(String::new());
            }
        }

        if let Some(role) = update_input.role {
            user.role = role;
        }

        if let Some(status) = update_input.status {
            user.status = status;
        }

        user.updated = chrono::Utc::now();

        // Update user in database
        self.repository.update(user).await
    }

    // Delete user
    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        // Check if user exists first
        self.get_user_by_id(id).await?;

        if !self.repository.delete(id).await? {
            return Err(AppError::user_not_found(id));
        }
        Ok(())
    }
}
