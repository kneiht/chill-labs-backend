use super::model::User;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, display_name, email, role, status, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, display_name, email, role as "role: _", status as "status: _", created, updated
            "#,
            user.id,
            user.display_name,
            user.email,
            user.role as _,
            user.status as _,
            user.created,
            user.updated
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, display_name, email, role as "role: _", status as "status: _", created, updated
            FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, display_name, email, role as "role: _", status as "status: _", created, updated
            FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, display_name, email, role as "role: _", status as "status: _", created, updated
            FROM users ORDER BY created DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update(&self, user: &User) -> Result<User> {
        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = $2, email = $3, role = $4, status = $5, updated = $6
            WHERE id = $1
            RETURNING id, display_name, email, role as "role: _", status as "status: _", created, updated
            "#,
            user.id,
            user.display_name,
            user.email,
            user.role as _,
            user.status as _,
            user.updated
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}