use crate::utils::password::verify_password;
use uuid::Uuid;

// Impor user model
use super::model::{Role, User, UserStatus};

// Import user repository
use super::repository::UserRepository;

// Import error handling
use crate::domain::error::AppError;

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

    pub async fn create_user(
        &self,
        display_name: String,
        username: String,
        email: Option<String>,
        password_hash: String,
        role: Role,
    ) -> Result<User, AppError> {
        // Validate input
        if display_name.trim().is_empty() {
            return Err(AppError::missing_field("display_name"));
        }

        if username.trim().is_empty() {
            return Err(AppError::missing_field("username"));
        }

        // Check if username already exists
        let existing_user = self.repository.find_by_username(&username[..]).await?;
        if existing_user.is_some() {
            return Err(AppError::username_already_exists(&username));
        }

        // Validate email if provided
        if let Some(ref email) = email {
            if email.trim().is_empty() {
                return Err(AppError::missing_field("email"));
            }

            // Basic email validation
            if !email.contains('@') {
                return Err(AppError::invalid_email_format(email));
            }

            // Check if email already exists
            if self.repository.find_by_email(email).await?.is_some() {
                return Err(AppError::email_already_exists(email));
            }
        }

        let user = User::new(display_name, username, email.unwrap_or_default(), password_hash, role);
        self.repository.create(user).await
    }

    pub async fn create_user_with_password(
        &self,
        display_name: String,
        username: String,
        email: Option<String>,
        password_hash: String,
        role: Role,
    ) -> Result<User, AppError> {
        self.create_user(display_name, username, email, password_hash, role)
            .await
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::user_not_found(id))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, AppError> {
        self.repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, AppError> {
        self.repository
            .find_by_username(username)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with username {} not found", username)))
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, AppError> {
        // For authentication, we need to use email since we're logging in with email
        let user = self.get_user_by_email(email).await?;

        if !verify_password(password, &user.password_hash)
            .map_err(|_| AppError::invalid_password())?
        {
            return Err(AppError::invalid_password());
        }

        Ok(user)
    }

    pub async fn authenticate_user_by_username(&self, username: &str, password: &str) -> Result<User, AppError> {
        let user = self.get_user_by_username(username).await?;

        if !verify_password(password, &user.password_hash)
            .map_err(|_| AppError::invalid_password())?
        {
            return Err(AppError::invalid_password());
        }

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all().await
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        display_name: Option<String>,
        username: Option<String>,
        email: Option<String>,
        role: Option<Role>,
        status: Option<UserStatus>,
    ) -> Result<User, AppError> {
        let mut user = self.get_user_by_id(id).await?;

        if let Some(display_name) = display_name {
            if display_name.trim().is_empty() {
                return Err(AppError::missing_field("display_name"));
            }
            user.display_name = display_name;
        }

        if let Some(username) = username {
            if username.trim().is_empty() {
                return Err(AppError::missing_field("username"));
            }

            // Check if new username conflicts
            if let Some(existing) = self.repository.find_by_username(&username[..]).await? {
                if existing.id != id {
                    return Err(AppError::username_already_exists(&username));
                }
            }
            user.username = username;
        }

        if let Some(email) = email {
            if !email.trim().is_empty() {
                if !email.contains('@') {
                    return Err(AppError::invalid_email_format(&email));
                }
                // Check if new email conflicts
                if let Some(existing) = self.repository.find_by_email(&email).await? {
                    if existing.id != id {
                        return Err(AppError::email_already_exists(&email));
                    }
                }
                user.email = email;
            } else {
                // If email is provided as empty string, set it to empty
                user.email = String::new();
            }
        }

        if let Some(role) = role {
            user.role = role;
        }

        if let Some(status) = status {
            user.status = status;
        }

        user.updated = chrono::Utc::now();

        self.repository.update(user).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        // Check if user exists first
        self.get_user_by_id(id).await?;

        if !self.repository.delete(id).await? {
            return Err(AppError::user_not_found(id));
        }
        Ok(())
    }
}
