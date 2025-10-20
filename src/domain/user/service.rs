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
        email: String,
        password_hash: String,
        role: Role,
    ) -> Result<User, AppError> {
        // Validate input
        if display_name.trim().is_empty() {
            return Err(AppError::missing_field("display_name"));
        }

        if email.trim().is_empty() {
            return Err(AppError::missing_field("email"));
        }

        // Basic email validation
        if !email.contains('@') {
            return Err(AppError::invalid_email_format(&email));
        }

        // Check if email already exists
        if self.repository.find_by_email(&email[..]).await?.is_some() {
            return Err(AppError::email_already_exists(&email));
        }

        let user = User::new(display_name, email, password_hash, role);
        self.repository.create(user).await
    }

    pub async fn create_user_with_password(
        &self,
        display_name: String,
        email: String,
        password_hash: String,
        role: Role,
    ) -> Result<User, AppError> {
        self.create_user(display_name, email, password_hash, role)
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

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, AppError> {
        let user = self.get_user_by_email(email).await?;

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

        if let Some(email) = email {
            if email.trim().is_empty() {
                return Err(AppError::missing_field("email"));
            }

            if !email.contains('@') {
                return Err(AppError::invalid_email_format(&email));
            }

            // Check if new email conflicts
            if let Some(existing) = self.repository.find_by_email(&email[..]).await? {
                if existing.id != id {
                    return Err(AppError::email_already_exists(&email));
                }
            }
            user.email = email;
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
