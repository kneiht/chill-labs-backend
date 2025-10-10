use crate::application::repositories::BaseRepository;
use crate::entities::HasId;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryRepository<T> {
    pub items: Mutex<Vec<T>>,
}

impl<T> InMemoryRepository<T> {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(Vec::new()),
        }
    }

    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            items: Mutex::new(items),
        }
    }
}

#[async_trait]
impl<T> BaseRepository<T> for InMemoryRepository<T>
where
    T: HasId + Clone + Send + Sync + 'static,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>> {
        let items = self.items.lock().unwrap();
        let item = items.iter().find(|item| item.id() == id).cloned();
        Ok(item)
    }

    async fn find_all(&self) -> Result<Vec<T>> {
        let items = self.items.lock().unwrap();
        Ok(items.clone())
    }

    async fn add(&self, entity: T) -> Result<T> {
        let mut items = self.items.lock().unwrap();
        items.push(entity.clone());
        Ok(entity)
    }

    async fn update(&self, entity: T) -> Result<T> {
        let mut items = self.items.lock().unwrap();
        let id = entity.id();
        if let Some(index) = items.iter().position(|item| item.id() == id) {
            items[index] = entity.clone();
            Ok(entity)
        } else {
            Err(anyhow::anyhow!("Entity not found"))
        }
    }

    async fn delete(&self, entity: T) -> Result<()> {
        let mut items = self.items.lock().unwrap();
        let id = entity.id();
        if let Some(index) = items.iter().position(|item| item.id() == id) {
            items.remove(index);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entity not found"))
        }
    }
}
