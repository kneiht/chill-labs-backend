use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::application::repositories::UserRepository;
use crate::application::services::JsonWebToken;
use crate::application::use_cases::auth::{CheckAuthUseCaseData, CheckAuthUseCaseDto};
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::Role;

pub struct CheckAuthUseCase<R: UserRepository + Send + Sync, J: JsonWebToken + Send + Sync> {
    pub json_web_token: Arc<J>,
    pub user_repository: Arc<R>,
}

#[async_trait]
impl<R: UserRepository + Send + Sync, J: JsonWebToken + Send + Sync>
    UseCase<CheckAuthUseCaseDto, CheckAuthUseCaseData> for CheckAuthUseCase<R, J>
{
    async fn execute(&self, input: CheckAuthUseCaseDto) -> UseCaseResponse<CheckAuthUseCaseData> {
        if let Err(e) = input.validate() {
            return UseCaseResponse::failure_unauthorized(
                "Input validation failed",
                Some(e.to_string()),
            );
        }

        // Verify token
        let payload = match self.json_web_token.verify(&input.token).await {
            Ok(p) => p,
            Err(e) => {
                return UseCaseResponse::failure_unauthorized("Invalid token", Some(e.to_string()));
            }
        };

        // Find user
        let id = match Uuid::parse_str(&payload.id) {
            Ok(id) => id,
            Err(_) => return UseCaseResponse::failure_unauthorized("Invalid token", None),
        };
        let user = match self.user_repository.find_by_id(id).await {
            Ok(Some(u)) => u,
            Ok(None) => return UseCaseResponse::failure_unauthorized("User not found", None),
            Err(e) => {
                return UseCaseResponse::failure_internal("Database error", Some(e.to_string()));
            }
        };

        // Check role
        let role_to_check = input.role_to_check.unwrap_or(Role::USER);
        if user.role != Role::ADMIN && user.role != role_to_check {
            return UseCaseResponse::failure_unauthorized("Insufficient permissions", None);
        }

        UseCaseResponse::success_ok(user, "Authentication successful")
    }
}
