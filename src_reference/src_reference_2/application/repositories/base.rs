use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BaseRepository<T> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn add(&self, entity: T) -> Result<T>;
    async fn update(&self, entity: T) -> Result<T>;
    async fn delete(&self, entity: T) -> Result<()>;
}
