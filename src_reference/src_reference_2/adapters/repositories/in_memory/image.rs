use crate::adapters::repositories::in_memory::{InMemoryRepository, seed_images};
use crate::application::repositories::BaseRepository;
use crate::entities::Image;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct ImageInMemoryRepository {
    pub base: InMemoryRepository<Image>,
}

impl ImageInMemoryRepository {
    pub async fn new() -> Self {
        let images = seed_images().await;
        Self {
            base: InMemoryRepository::with_items(images),
        }
    }
}

#[async_trait]
impl BaseRepository<Image> for ImageInMemoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Image>> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<Image>> {
        self.base.find_all().await
    }

    async fn add(&self, entity: Image) -> Result<Image> {
        self.base.add(entity).await
    }

    async fn update(&self, entity: Image) -> Result<Image> {
        self.base.update(entity).await
    }

    async fn delete(&self, entity: Image) -> Result<()> {
        self.base.delete(entity).await
    }
}
