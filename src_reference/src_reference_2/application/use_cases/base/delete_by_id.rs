use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::repositories::BaseRepository;
use crate::application::use_cases::{UseCase, UseCaseResponse};
use crate::entities::Entity;

pub struct DeleteByIdUseCase<R, E> {
    pub repository: Arc<R>,
    pub _phantom: PhantomData<E>,
}

#[async_trait]
impl<R, E> UseCase<String, ()> for DeleteByIdUseCase<R, E>
where
    R: BaseRepository<E> + Send + Sync,
    E: Entity + Send + Sync,
{
    async fn execute(&self, input: String) -> UseCaseResponse<()> {
        let id = match Uuid::parse_str(&input) {
            Ok(id) => id,
            Err(_) => return UseCaseResponse::failure_validation("Invalid ID", None),
        };
        let entity = match self.repository.find_by_id(id).await {
            Ok(Some(entity)) => entity,
            Ok(None) => return UseCaseResponse::failure_not_found("Entity not found", None),
            Err(_) => {
                return UseCaseResponse::failure_internal("Failed to find entity", None);
            }
        };
        if let Err(_) = self.repository.delete(entity).await {
            return UseCaseResponse::failure_internal("Failed to delete entity", None);
        }
        UseCaseResponse::success_no_content("Entity deleted successfully")
    }
}
