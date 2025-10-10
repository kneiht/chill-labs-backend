use anyhow::Result;
use async_trait::async_trait;

pub mod auth;
pub mod base;
pub mod response;
pub mod uploads;
pub mod user;

// Re-export for convenience
pub use auth::*;
pub use base::*;
pub use response::*;
pub use uploads::*;
pub use user::*;

#[async_trait]
pub trait UseCase<Input, Output> {
    async fn execute(&self, input: Input) -> UseCaseResponse<Output>;
}
