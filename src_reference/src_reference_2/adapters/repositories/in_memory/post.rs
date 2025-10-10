use crate::adapters::repositories::in_memory::{InMemoryRepository, seed_posts};
use crate::application::repositories::BaseRepository;
use crate::entities::Post;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct PostInMemoryRepository {
    pub base: InMemoryRepository<Post>,
}

impl PostInMemoryRepository {
    pub async fn new() -> Self {
        let posts = seed_posts().await;
        Self {
            base: InMemoryRepository::with_items(posts),
        }
    }
}

#[async_trait]
impl BaseRepository<Post> for PostInMemoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<Post>> {
        self.base.find_all().await
    }

    async fn add(&self, entity: Post) -> Result<Post> {
        self.base.add(entity).await
    }

    async fn update(&self, entity: Post) -> Result<Post> {
        self.base.update(entity).await
    }

    async fn delete(&self, entity: Post) -> Result<()> {
        self.base.delete(entity).await
    }
}
