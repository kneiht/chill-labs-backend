use crate::utils::password::verify_password;
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

// Input structs for service methods

#[derive(Debug, Validate)]
pub struct CreateUserInput {
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: String,

    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    pub password_hash: String,

    pub role: Role,
}

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

#[derive(Debug, Validate)]
pub struct AuthenticateUserInput {
    #[validate(length(min = 1, message = "Email cannot be empty"))]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct UpdateUserData {
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: Option<String>,

    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    pub role: Option<Role>,

    pub status: Option<UserStatus>,
}

#[derive(Debug, Validate)]
pub struct AuthenticateUserByUsernameInput {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

// Implement Transformer for CreateUserInput
impl Transformer<User> for CreateUserInput {
    fn transform(self) -> Result<User, AppError> {
        // Validate the input
        self.validate().map_err(|e| AppError::validation(&e.to_string()))?;

        // Create the user
        Ok(User::new(
            self.display_name,
            self.username,
            self.email.unwrap_or_default(),
            self.password_hash,
            self.role,
        ))
    }
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

    pub async fn create_user<I: Transformer<User>>(&self, input: I) -> Result<User, AppError> {
        let user = input.transform()?;

        // Check if username already exists
        if self.repository.find_by_username(&user.username[..]).await?.is_some() {
            return Err(AppError::username_already_exists(&user.username));
        }

        // Check if email already exists if provided
        if !user.email.is_empty() {
            if self.repository.find_by_email(&user.email[..]).await?.is_some() {
                return Err(AppError::email_already_exists(&user.email));
            }
        }

        self.repository.create(user).await
    }

    pub async fn create_user_with_password<I: Transformer<User>>(&self, input: I) -> Result<User, AppError> {
        self.create_user(input).await
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

    pub async fn authenticate_user<I: Transformer<AuthenticateUserInput>>(&self, input: I) -> Result<User, AppError> {
        let auth_input = input.transform()?;
        auth_input.validate().map_err(|e| AppError::validation(&e.to_string()))?;

        // For authentication, we need to use email since we're logging in with email
        let user = self.get_user_by_email(&auth_input.email).await?;

        if !verify_password(&auth_input.password, &user.password_hash)
            .map_err(|_| AppError::invalid_password())?
        {
            return Err(AppError::invalid_password());
        }

        Ok(user)
    }

    pub async fn authenticate_user_by_username<I: Transformer<AuthenticateUserByUsernameInput>>(&self, input: I) -> Result<User, AppError> {
        let auth_input = input.transform()?;
        auth_input.validate().map_err(|e| AppError::validation(&e.to_string()))?;

        let user = self.get_user_by_username(&auth_input.username).await?;

        if !verify_password(&auth_input.password, &user.password_hash)
            .map_err(|_| AppError::invalid_password())?
        {
            return Err(AppError::invalid_password());
        }

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all().await
    }

    pub async fn update_user<I: Transformer<UpdateUserInput>>(&self, input: I) -> Result<User, AppError> {
        let update_input = input.transform()?;
        update_input.validate().map_err(|e| AppError::validation(&e.to_string()))?;

        let mut user = self.get_user_by_id(update_input.id).await?;

        if let Some(display_name) = update_input.display_name {
            user.display_name = display_name;
        }

        if let Some(username) = update_input.username {
            // Check if new username conflicts
            if let Some(existing) = self.repository.find_by_username(&username[..]).await? {
                if existing.id != update_input.id {
                    return Err(AppError::username_already_exists(&username));
                }
            }
            user.username = username;
        }

        if let Some(email) = update_input.email {
            if !email.trim().is_empty() {
                // Check if new email conflicts
                if let Some(existing) = self.repository.find_by_email(email.as_str()).await? {
                    if existing.id != update_input.id {
                        return Err(AppError::email_already_exists(&email));
                    }
                }
                user.email = email;
            } else {
                // If email is provided as empty string, set it to empty
                user.email = String::new();
            }
        }

        if let Some(role) = update_input.role {
            user.role = role;
        }

        if let Some(status) = update_input.status {
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
