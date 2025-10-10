use async_trait::async_trait;
use serde::de::DeserializeOwned;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

pub mod base_entity;
pub mod image_entity;
pub mod post_entity;
pub mod user_entity;

// Re-export for convenience
pub use base_entity::*;
pub use image_entity::*;
pub use post_entity::*;
pub use user_entity::*;

pub trait HasId {
    fn id(&self) -> Uuid;
}

#[async_trait]
pub trait Entity: Send + Sync {
    type CreateDto: DeserializeOwned + Validate + Send + Sync;
    type UpdateDto: DeserializeOwned + Validate + Send + Sync + HasId;
    async fn create(dto: Self::CreateDto) -> Result<Self, EntityError>
    where
        Self: Sized;
    fn update(&mut self, dto: &Self::UpdateDto);
}

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("Validation failed: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Hashing failed: {0}")]
    Hash(#[from] bcrypt::BcryptError),
}
