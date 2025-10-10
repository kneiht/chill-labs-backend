use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::application::repositories::BaseRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::Entity;

pub struct GetAllUseCase<R, E> {
    pub repository: Arc<R>,
    pub _phantom: PhantomData<E>,
}

#[async_trait]
impl<R, E> UseCase<(), Vec<E>> for GetAllUseCase<R, E>
where
    R: BaseRepository<E> + Send + Sync,
    E: Entity + Send + Sync,
{
    async fn execute(&self, _input: ()) -> UseCaseResponse<Vec<E>> {
        let entities = match self.repository.find_all().await {
            Ok(entities) => entities,
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to retrieve entities", None);
            }
        };
        UseCaseResponse::success_ok(entities, "Entities retrieved successfully")
    }
}
