use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use crate::application::repositories::UserRepository;
use crate::application::services::JsonWebToken;
use crate::application::use_cases::auth::{LoginUseCaseData, LoginUseCaseDto, TokenPair};
use crate::application::use_cases::{UseCase, UseCaseResponse};

#[derive(Clone)]
pub struct LoginUseCase<R: UserRepository + Send + Sync, J: JsonWebToken + Send + Sync> {
    pub user_repository: Arc<R>,
    pub json_web_token: Arc<J>,
}

#[async_trait]
impl<R: UserRepository + Send + Sync, J: JsonWebToken + Send + Sync>
    UseCase<LoginUseCaseDto, LoginUseCaseData> for LoginUseCase<R, J>
{
    async fn execute(&self, input: LoginUseCaseDto) -> UseCaseResponse<LoginUseCaseData> {
        if let Err(e) = input.validate() {
            return UseCaseResponse::failure_unauthorized(
                "Input validation failed",
                Some(e.to_string()),
            );
        }

        // Find user
        let user = match self.user_repository.find_by_email(&input.email).await {
            Ok(Some(u)) => u,
            Ok(None) => {
                return UseCaseResponse::failure_unauthorized("Invalid email or password", None);
            }
            Err(e) => {
                return UseCaseResponse::failure_internal("Database error", Some(e.to_string()));
            }
        };

        // Verify password
        let password_valid = match user.verify_password(&input.password) {
            Ok(v) => v,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Password verification error",
                    Some(e.to_string()),
                );
            }
        };
        if !password_valid {
            return UseCaseResponse::failure_unauthorized("Invalid email or password", None);
        }

        // Create payload
        let payload = crate::application::services::JwtPayload {
            id: user.base.id.to_string(),
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            exp: None,
        };

        // Sign tokens (assume OneHour for access, SevenDays for refresh)
        let access_token = match self
            .json_web_token
            .sign(
                payload.clone(),
                crate::application::services::ExpiresIn::OneHour,
            )
            .await
        {
            Ok(t) => t,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Token signing error",
                    Some(e.to_string()),
                );
            }
        };
        let refresh_token = match self
            .json_web_token
            .sign(payload, crate::application::services::ExpiresIn::SevenDays)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                return UseCaseResponse::failure_internal(
                    "Token signing error",
                    Some(e.to_string()),
                );
            }
        };

        let data = LoginUseCaseData {
            user,
            token: TokenPair {
                access_token,
                refresh_token,
            },
        };

        UseCaseResponse::success_ok(data, "Login successful")
    }
}
