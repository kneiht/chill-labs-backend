use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

// Import user model
use super::model::{User, UserRow};

// UserRepository struct
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

// Implementation of UserRepository
impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User> {
        let result = sqlx::query_as!(
             UserRow,
             r#"
              INSERT INTO users (id, display_name, email, password_hash, role, status, created, updated)
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
              RETURNING id, display_name, email, password_hash, role, status, created, updated
              "#,
             user.id,
             user.display_name,
             user.email,
             user.password_hash,
             format!("{:?}", user.role),
             format!("{:?}", user.status),
             user.created,
             user.updated
         )
         .fetch_one(&self.pool)
         .await?;

        Ok(result.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
              SELECT id, display_name, email, password_hash, role, status, created, updated
              FROM users WHERE id = $1
              "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|u| u.into()))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
              SELECT id, display_name, email, password_hash, role, status, created, updated
              FROM users WHERE email = $1
              "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|u| u.into()))
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            UserRow,
            r#"
              SELECT id, display_name, email, password_hash, role, status, created, updated
              FROM users ORDER BY created DESC
              "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn update(&self, user: &User) -> Result<User> {
        let result = sqlx::query_as!(
             UserRow,
             r#"
              UPDATE users
              SET display_name = $2, email = $3, password_hash = $4, role = $5, status = $6, updated = $7
              WHERE id = $1
              RETURNING id, display_name, email, password_hash, role, status, created, updated
              "#,
             user.id,
             user.display_name,
             user.email,
             user.password_hash,
             format!("{:?}", user.role),
             format!("{:?}", user.status),
             user.updated
         )
         .fetch_one(&self.pool)
         .await?;

        Ok(result.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
