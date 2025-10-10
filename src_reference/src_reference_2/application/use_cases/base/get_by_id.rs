use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::repositories::BaseRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::Entity;

pub struct GetByIdUseCase<R, E> {
    pub repository: Arc<R>,
    pub _phantom: PhantomData<E>,
}

#[async_trait]
impl<R, E> UseCase<String, Option<E>> for GetByIdUseCase<R, E>
where
    R: BaseRepository<E> + Send + Sync,
    E: Entity + Send + Sync,
{
    async fn execute(&self, input: String) -> UseCaseResponse<Option<E>> {
        let id = match Uuid::parse_str(&input) {
            Ok(id) => id,
            Err(_) => return UseCaseResponse::failure_validation("Invalid ID", None),
        };
        let entity = match self.repository.find_by_id(id).await {
            Ok(entity) => entity,
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to retrieve entity", None);
            }
        };
        UseCaseResponse::success_ok(entity, "Entity retrieved successfully")
    }
}
