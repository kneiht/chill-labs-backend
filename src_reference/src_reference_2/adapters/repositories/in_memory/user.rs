use crate::adapters::repositories::in_memory::{InMemoryRepository, seed_users};
use crate::application::repositories::{BaseRepository, UserRepository};
use crate::entities::User;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct UserInMemoryRepository {
    pub base: InMemoryRepository<User>,
}

impl UserInMemoryRepository {
    pub async fn new() -> Self {
        let users = seed_users().await;
        Self {
            base: InMemoryRepository::with_items(users),
        }
    }
}

#[async_trait]
impl BaseRepository<User> for UserInMemoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        self.base.find_all().await
    }

    async fn add(&self, entity: User) -> Result<User> {
        self.base.add(entity).await
    }

    async fn update(&self, entity: User) -> Result<User> {
        self.base.update(entity).await
    }

    async fn delete(&self, entity: User) -> Result<()> {
        self.base.delete(entity).await
    }
}

#[async_trait]
impl UserRepository for UserInMemoryRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let items = self.base.items.lock().unwrap();
        let user = items.iter().find(|u| u.email == email).cloned();
        Ok(user)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>> {
        let items = self.base.items.lock().unwrap();
        let user = items
            .iter()
            .find(|u| u.name.as_ref() == Some(&name.to_string()))
            .cloned();
        Ok(user)
    }
}
