use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use crate::application::repositories::UserRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::{CreateUserDto, User};

pub struct AddUserUseCase<R: UserRepository + Send + Sync> {
    pub user_repository: Arc<R>,
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<CreateUserDto, User> for AddUserUseCase<R> {
    async fn execute(&self, input: CreateUserDto) -> UseCaseResponse<User> {
        // Validate input
        if let Err(e) = input.validate() {
            return UseCaseResponse::failure_validation(
                "Input validation failed",
                Some(e.to_string()),
            );
        }

        // Check if user with email exists
        let existing_email = match self.user_repository.find_by_email(&input.email).await {
            Ok(e) => e,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Failed to find user",
                    Some(e.to_string()),
                );
            }
        };
        if existing_email.is_some() {
            return UseCaseResponse::failure_conflict("User with this email already exists", None);
        }

        // Check if user with name exists
        let existing_name = match self.user_repository.find_by_name(&input.name).await {
            Ok(e) => e,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Failed to find user",
                    Some(e.to_string()),
                );
            }
        };
        if existing_name.is_some() {
            return UseCaseResponse::failure_conflict("User with this name already exists", None);
        }

        // Create user
        let user = match User::create(input).await {
            Ok(u) => u,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "User creation failed",
                    Some(e.to_string()),
                );
            }
        };

        // Add to repository
        let new_user = match self.user_repository.add(user).await {
            Ok(u) => u,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Failed to add user",
                    Some(e.to_string()),
                );
            }
        };

        UseCaseResponse::success_created(new_user, "User created successfully")
    }
}
