use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::application::repositories::BaseRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::{Entity, EntityError};

pub struct AddUseCase<R, E> {
    pub repository: Arc<R>,
    pub _phantom: PhantomData<E>,
}

#[async_trait]
impl<R, E> UseCase<E::CreateDto, E> for AddUseCase<R, E>
where
    R: BaseRepository<E> + Send + Sync,
    E: Entity + Send + Sync,
    E::CreateDto: validator::Validate,
{
    async fn execute(&self, input: E::CreateDto) -> UseCaseResponse<E> {
        let entity = match E::create(input).await {
            Ok(entity) => entity,
            Err(EntityError::Validation(ref validation_errors)) => {
                return UseCaseResponse::failure_validation(
                    "Input validation failed",
                    Some(validation_errors.to_string()),
                );
            }
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to create entity", None);
            }
        };
        let new_entity = match self.repository.add(entity).await {
            Ok(new_entity) => new_entity,
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to add entity", None);
            }
        };
        UseCaseResponse::success_created(new_entity, "Entity created successfully")
    }
}
