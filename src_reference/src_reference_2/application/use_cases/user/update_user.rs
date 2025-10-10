use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use crate::application::repositories::UserRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::{UpdateUserDto, User};

pub struct UpdateUserUseCase<R: UserRepository + Send + Sync> {
    pub user_repository: Arc<R>,
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<UpdateUserDto, User> for UpdateUserUseCase<R> {
    async fn execute(&self, input: UpdateUserDto) -> UseCaseResponse<User> {
        // Validate input
        if let Err(e) = input.validate() {
            return UseCaseResponse::failure_validation(
                "Input validation failed",
                Some(e.to_string()),
            );
        }

        let UpdateUserDto { id, name, role } = input;

        // Find user
        let mut user = match self.user_repository.find_by_id(id).await {
            Ok(Some(u)) => u,
            Ok(None) => return UseCaseResponse::failure_not_found("User not found", None),
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Failed to find user",
                    Some(e.to_string()),
                );
            }
        };

        // Check name conflict
        if let Some(new_name) = &name {
            let existing = match self.user_repository.find_by_name(new_name).await {
                Ok(e) => e,
                Err(e) => {
                    return UseCaseResponse::failure_internal(
                        "Failed to find user",
                        Some(e.to_string()),
                    );
                }
            };
            if let Some(existing) = existing {
                if existing.base.id != id {
                    return UseCaseResponse::failure_conflict(
                        "User with this name already exists",
                        None,
                    );
                }
            }
        }

        // Update fields
        if let Some(n) = name {
            user.name = Some(n);
        }
        if let Some(r) = role {
            user.role = r;
        }
        user.base.updated_at = chrono::Utc::now();

        // Update in repository
        let updated_user = match self.user_repository.update(user).await {
            Ok(u) => u,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Failed to update user",
                    Some(e.to_string()),
                );
            }
        };

        UseCaseResponse::success_ok(updated_user, "User updated successfully")
    }
}
