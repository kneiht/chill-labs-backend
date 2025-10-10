use super::error::UserServiceError;
use super::model::{Gender, Membership, User, UserRole, UserStatus};
use super::repository::{
    CreateUserRepoInput, UpdatePasswordRepoInput, UpdateUserRepoInput, UserRepository,
};
use crate::utils::email::EmailService;
use crate::utils::password::{hash_password, verify_password};

use chrono::{Duration, NaiveDate, Utc};
use rand::{distr::Alphanumeric, Rng};
use std::sync::Arc;
use uuid::Uuid;

// Define the UserService struct
pub struct UserService {
    pub repository: UserRepository,
    email_service: Option<Arc<EmailService>>,
}

// Define the DTO CreateUserServiceInput struct
pub struct CreateUserServiceInput {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

// Define the DTO AuthUserInput struct
pub struct AuthUserInput {
    pub email: String,
    pub password: String,
}

// Define the DTO ChangePasswordInput struct
pub struct ChangePasswordInput {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}

// Define the DTO UpdateProfileInput struct
pub struct UpdateProfileInput {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub avatar_url: Option<Option<String>>,
    pub status: Option<UserStatus>,
    pub role: Option<UserRole>,
    pub membership: Option<Membership>,
    pub gender: Option<Gender>,
    pub date_of_birth: Option<Option<NaiveDate>>,
    pub phone: Option<Option<String>>,
    pub bio: Option<Option<String>>,
}

// Define DTO UserRepoOutput struct which is the same as User struct
pub type UserServiceOutput = User;

// Implement the UserService struct
impl UserService {
    // Constructor for the UserService struct
    pub fn new(repository: UserRepository) -> Self {
        Self {
            repository,
            email_service: None,
        }
    }

    // Set the email service
    pub fn with_email_service(mut self, email_service: Arc<EmailService>) -> Self {
        self.email_service = Some(email_service);
        self
    }

    // Create a new user
    pub async fn create_user(
        &self,
        input: CreateUserServiceInput,
    ) -> Result<UserServiceOutput, UserServiceError> {
        // Check if user already exists
        if self
            .repository
            .get_by_email(&input.email)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .is_some()
        {
            return Err(UserServiceError::UserAlreadyExists { email: input.email });
        }

        // Hash the password
        let password_hash = hash_password(&input.password)
            .map_err(|e| UserServiceError::InternalError { source: e })?;

        // Prepare input for repository
        let repo_input = CreateUserRepoInput {
            email: input.email,
            password_hash,
            display_name: input.display_name,
        };
        // Create the user
        let user = self
            .repository
            .create(repo_input)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;
        Ok(user)
    }

    // Authenticate a user
    pub async fn authenticate_user(
        &self,
        input: AuthUserInput,
    ) -> Result<UserServiceOutput, UserServiceError> {
        // Find user by email
        let user = self
            .repository
            .get_by_email(&input.email)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or(UserServiceError::InvalidCredentials)?;
        verify_password(&input.password, &user.password_hash)
            .map_err(|_| UserServiceError::InvalidCredentials)?;
        // Update last login time
        self.repository
            .update_last_login(user.id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;
        Ok(user)
    }

    // Change a user's password
    pub async fn change_password(
        &self,
        input: ChangePasswordInput,
    ) -> Result<UserServiceOutput, UserServiceError> {
        // Find user by ID
        let user = self
            .repository
            .get_by_id(input.user_id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or(UserServiceError::UserNotFound)?;
        // Verify current password
        verify_password(&input.current_password, &user.password_hash)
            .map_err(|_| UserServiceError::InvalidCurrentPassword)?;
        // Hash the new password
        let new_password_hash = hash_password(&input.new_password)
            .map_err(|e| UserServiceError::InternalError { source: e })?;
        // Prepare input for repository
        let update_input = UpdatePasswordRepoInput {
            id: user.id,
            password_hash: new_password_hash,
        };
        // Update the password
        let updated_user = self
            .repository
            .update_password(update_input)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;
        Ok(updated_user)
    }

    // Get a user by id
    pub async fn get_user_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<UserServiceOutput, UserServiceError> {
        self.repository
            .get_by_id(user_id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or_else(|| UserServiceError::UserNotFound)
    }

    // Get a user by email
    pub async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<UserServiceOutput, UserServiceError> {
        self.repository
            .get_by_email(email)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or_else(|| UserServiceError::UserNotFound)
    }

    // Update a user's profile
    pub async fn update_profile(
        &self,
        input: UpdateProfileInput,
    ) -> Result<UserServiceOutput, UserServiceError> {
        // Fetch the current user data first to handle optional updates
        let current_user = self
            .repository
            .get_by_id(input.user_id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or(UserServiceError::UserNotFound)?;

        // Prepare input for the repository, merging new values with existing ones
        let repo_input = UpdateUserRepoInput {
            id: input.user_id,
            display_name: input.display_name.unwrap_or(current_user.display_name),
            avatar_url: match input.avatar_url {
                Some(Some(url)) => Some(url),    // Set new URL
                Some(None) => None,              // Set to NULL
                None => current_user.avatar_url, // Keep existing
            },
            status: input.status.unwrap_or(current_user.status),
            role: input.role.unwrap_or(current_user.role),
            membership: input.membership.unwrap_or(current_user.membership),
            gender: input.gender.unwrap_or(current_user.gender),
            date_of_birth: match input.date_of_birth {
                Some(Some(dob_val)) => Some(dob_val),
                Some(None) => None,
                None => current_user.date_of_birth,
            },
            phone: match input.phone {
                Some(Some(p_val)) => Some(p_val),
                Some(None) => None,
                None => current_user.phone,
            },
            bio: match input.bio {
                Some(Some(b_val)) => Some(b_val),
                Some(None) => None,
                None => current_user.bio,
            },
        };

        // Call the repository update method
        let updated_user = self
            .repository
            .update_user(repo_input)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;
        Ok(updated_user)
    }

    // Generate a random verification token
    fn generate_verification_token(&self) -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    // Send verification email
    pub async fn send_verification_email(&self, user_id: Uuid) -> Result<(), UserServiceError> {
        // Get the user
        let user = self
            .repository
            .get_by_id(user_id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or(UserServiceError::UserNotFound)?;

        // Check if user's email is already verified
        if user.email_verified {
            return Err(UserServiceError::EmailAlreadyVerified);
        }

        // Check if email service is available
        let email_service_clone = self.email_service.as_ref().cloned().ok_or_else(|| {
            UserServiceError::InternalError {
                source: anyhow::Error::msg("Email service not configured"),
            }
        })?;

        // Generate a verification token
        let token = self.generate_verification_token();
        let token_clone = token.clone(); // Clone token for the spawned task

        // Set expiration time (24 hours from now)
        let expires_at = Utc::now() + Duration::hours(24);

        // Clone user details for the spawned task
        let user_email_clone = user.email.clone();
        let user_display_name_clone = user.display_name.clone();

        // It's crucial that database operations that MUST succeed for the user's
        // action to be valid (like creating the token) happen *before* spawning
        // the background task, or are part of a transaction that ensures atomicity
        // if the background task also interacts with the DB for this specific operation.

        // Create a verification token in the database
        self.repository
            .create_email_verification_token(user_id, &token, expires_at)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;

        // Send the verification email
        // Offload email sending to a background task
        tokio::spawn(async move {
            tracing::info!(
                "Attempting to send verification email to {} in background (spawned task).",
                user_email_clone
            );

            let send_result = email_service_clone
                .send_verification_email(&user_email_clone, &user_display_name_clone, &token_clone)
                .await;

            match send_result {
                Ok(_) => {
                    tracing::info!(
                        target_email = %user_email_clone,
                        "Successfully processed sending verification email in background (spawned task)."
                    );
                }
                Err(e) => {
                    // anyhow::Error's Debug or Display representation often includes the cause chain.
                    tracing::error!(target_email = %user_email_clone, error = ?e, "Failed to send verification email in background (spawned task).");
                }
            }
        });

        Ok(())
    }

    // Verify email with token
    pub async fn verify_email_with_token(&self, token: &str) -> Result<User, UserServiceError> {
        // Get the token from the database
        let verification_token = self
            .repository
            .get_email_verification_token(token)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?
            .ok_or_else(|| UserServiceError::InvalidVerificationToken)?;

        // Check if the token is expired
        if verification_token.expires_at < Utc::now() {
            return Err(UserServiceError::VerificationTokenExpired);
        }

        // Update the user's email verification status
        let user = self
            .repository
            .update_email_verified_status(verification_token.user_id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;

        // Delete the token
        self.repository
            .delete_email_verification_token(verification_token.id)
            .await
            .map_err(|e| UserServiceError::RepositoryError { source: e })?;

        Ok(user)
    }
}
