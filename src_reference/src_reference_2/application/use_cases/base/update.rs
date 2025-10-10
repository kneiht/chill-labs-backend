use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::application::repositories::BaseRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::{Entity, HasId};

pub struct UpdateUseCase<R, E> {
    pub repository: Arc<R>,
    pub _phantom: PhantomData<E>,
}

#[async_trait]
impl<R, E> UseCase<E::UpdateDto, E> for UpdateUseCase<R, E>
where
    R: BaseRepository<E> + Send + Sync,
    E: Entity + Send + Sync,
{
    async fn execute(&self, input: E::UpdateDto) -> UseCaseResponse<E> {
        let id = input.id();
        let mut entity = match self.repository.find_by_id(id).await {
            Ok(Some(entity)) => entity,
            Ok(None) => return UseCaseResponse::failure_not_found("Entity not found", None),
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to find entity", None);
            }
        };
        // TODO: Validate input
        entity.update(&input);
        let updated = match self.repository.update(entity).await {
            Ok(updated) => updated,
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to update entity", None);
            }
        };
        UseCaseResponse::success_ok(updated, "Entity updated successfully")
    }
}
