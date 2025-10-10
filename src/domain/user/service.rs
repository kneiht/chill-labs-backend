use super::model::{Role, User, UserStatus};
use super::repository::UserRepository;
use anyhow::{anyhow, Result};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, display_name: String, email: String, role: Role) -> Result<User> {
        // Check if email already exists
        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(anyhow!("Email already exists"));
        }

        let user = User::new(display_name, email, role);
        self.repository.create(&user).await
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("User not found"))
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        self.repository.find_all().await
    }

    pub async fn update_user(&self, id: Uuid, display_name: Option<String>, email: Option<String>, role: Option<Role>, status: Option<UserStatus>) -> Result<User> {
        let mut user = self.get_user_by_id(id).await?;

        if let Some(display_name) = display_name {
            user.display_name = display_name;
        }

        if let Some(email) = email {
            // Check if new email conflicts
            if let Some(existing) = self.repository.find_by_email(&email).await? {
                if existing.id != id {
                    return Err(anyhow!("Email already exists"));
                }
            }
            user.email = email;
        }

        if let Some(role) = role {
            user.role = role;
        }

        if let Some(status) = status {
            user.status = status;
        }

        user.updated = chrono::Utc::now();

        self.repository.update(&user).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        if !self.repository.delete(id).await? {
            return Err(anyhow!("User not found"));
        }
        Ok(())
    }
}

