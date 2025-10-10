use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use crate::application::services::JsonWebToken;
use crate::application::use_cases::auth::{RegisterUseCaseData, RegisterUseCaseDto, TokenPair};
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::CreateUserDto;

use crate::application::use_cases::UseCase as AddUseCaseTrait;

#[derive(Clone)]
pub struct RegisterUseCase<
    J: JsonWebToken + Send + Sync,
    A: AddUseCaseTrait<CreateUserDto, crate::entities::User> + Send + Sync,
> {
    pub json_web_token: Arc<J>,
    pub add_user_use_case: Arc<A>,
}

#[async_trait]
impl<
    J: JsonWebToken + Send + Sync,
    A: AddUseCaseTrait<CreateUserDto, crate::entities::User> + Send + Sync,
> UseCase<RegisterUseCaseDto, RegisterUseCaseData> for RegisterUseCase<J, A>
{
    async fn execute(&self, input: RegisterUseCaseDto) -> UseCaseResponse<RegisterUseCaseData> {
        if let Err(e) = input.validate() {
            return UseCaseResponse::failure_unauthorized(
                "Input validation failed",
                Some(e.to_string()),
            );
        }

        let name = match input.name.clone() {
            Some(n) => n,
            None => return UseCaseResponse::failure_validation("Name is required", None),
        };

        // Create CreateUserDto
        let create_dto = CreateUserDto {
            name,
            email: input.email.clone(),
            password: input.password.clone(),
            role: input.role,
        };

        // Execute add user
        let add_response = self.add_user_use_case.execute(create_dto).await;
        if !add_response.success {
            return UseCaseResponse::<RegisterUseCaseData> {
                success: false,
                message: add_response.message,
                status: add_response.status,
                data: None,
                pagination: None,
                error: add_response.error,
            };
        }
        let user = add_response.data.unwrap();

        // Create payload
        let payload = crate::application::services::JwtPayload {
            id: user.base.id.to_string(),
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            exp: None,
        };

        // Sign tokens
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

        let data = RegisterUseCaseData {
            user,
            token: TokenPair {
                access_token,
                refresh_token,
            },
        };

        UseCaseResponse::success_created(data, "Registration successful")
    }
}
