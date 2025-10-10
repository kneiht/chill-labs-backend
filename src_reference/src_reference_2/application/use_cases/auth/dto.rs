use crate::entities::{Role, User};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterUseCaseDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginUseCaseDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CheckAuthUseCaseDto {
    pub token: String,
    pub role_to_check: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUseCaseData {
    pub user: User,
    pub token: TokenPair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUseCaseData {
    pub user: User,
    pub token: TokenPair,
}

pub type CheckAuthUseCaseData = User;
