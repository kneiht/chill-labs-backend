use crate::application::repositories::BaseRepository;
use crate::entities::User;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: BaseRepository<User> {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<User>>;
}
