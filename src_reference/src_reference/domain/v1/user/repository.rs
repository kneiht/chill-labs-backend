use super::model::{EmailVerificationToken, Gender, Membership, User, UserRole, UserStatus};

use anyhow::Context;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

// Define the UserRepository struct
pub struct UserRepository {
    pub pool: PgPool,
}

// Define the DTO CreateUserRepoInput struct
pub struct CreateUserRepoInput {
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
}

// Define DTO UpdatePasswordRepoInput struct
pub struct UpdatePasswordRepoInput {
    pub id: Uuid,
    pub password_hash: String,
}

// Define DTO UpdateUserInput struct
pub struct UpdateUserRepoInput {
    pub id: Uuid,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub membership: Membership,
    pub gender: Gender,
    pub date_of_birth: Option<NaiveDate>,
    pub phone: Option<String>,
    pub bio: Option<String>,
}

// Define DTO UserRepoOutput struct which is the same as User struct
pub type UserRepoOutput = User;

// Implement the UserRepository struct
impl UserRepository {
    // Constructor for the UserRepository struct
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Create a new user
    pub async fn create(&self, input: CreateUserRepoInput) -> anyhow::Result<UserRepoOutput> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            INSERT INTO users (email, password_hash, display_name, created, updated)
            VALUES ($1, $2, $3, $4, $5) -- New fields will be NULL by default if not specified
            RETURNING id, email, password_hash, display_name, status as "status: UserStatus", 
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            "#,
            input.email,
            input.password_hash,
            input.display_name,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create user in database")?;
        Ok(user)
    }

    // Get a user by email
    pub async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<UserRepoOutput>> {
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            SELECT id, email, password_hash, display_name, status as "status: UserStatus", 
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!(
            "Failed to fetch user by email {} from database",
            email
        ))?;
        Ok(user)
    }

    // Get a user by id
    pub async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<UserRepoOutput>> {
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            SELECT id, email, password_hash, display_name, status as "status: UserStatus", 
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("Failed to fetch user by id {} from database", id))?;

        Ok(user)
    }

    // Update a user's password
    pub async fn update_password(
        &self,
        input: UpdatePasswordRepoInput,
    ) -> anyhow::Result<UserRepoOutput> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            UPDATE users
            SET password_hash = $1, updated = $2
            WHERE id = $3
            RETURNING id, email, password_hash, display_name, status as "status: UserStatus", 
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            "#,
            input.password_hash,
            now,
            input.id
        )
        .fetch_one(&self.pool)
        .await
        .context(format!(
            "Failed to update password for user id {} in database",
            input.id
        ))?;
        Ok(user)
    }

    // Update user info
    pub async fn update_user(&self, input: UpdateUserRepoInput) -> anyhow::Result<UserRepoOutput> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            UPDATE users
            SET display_name = $1,
                avatar_url = $2,
                status = $3,
                role = $4,
                membership = $5,
                gender = $6,
                date_of_birth = $7,
                phone = $8,
                bio = $9,
                updated = $10
            WHERE id = $11
            RETURNING id, email, password_hash, display_name, status as "status: UserStatus",
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            "#,
            input.display_name,
            input.avatar_url,
            input.status as UserStatus,
            input.role as UserRole,
            input.membership as Membership,
            input.gender as Gender,
            input.date_of_birth,
            input.phone,
            input.bio,
            now,
            input.id
        )
        .fetch_one(&self.pool)
        .await
        .context(format!(
            "Failed to update user info for user id {} in database",
            input.id
        ))?;
        Ok(user)
    }

    // Update a user's email verification status
    pub async fn update_email_verified_status(&self, id: Uuid) -> anyhow::Result<UserRepoOutput> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            UserRepoOutput,
            r#"
            UPDATE users
            SET email_verified = $1, updated = $2, status = 'active'
            WHERE id = $3
            RETURNING id, email, password_hash, display_name, status as "status: UserStatus",
            role as "role: UserRole", email_verified, avatar_url, created, updated, last_login,
            membership as "membership: Membership", gender as "gender: Gender",
            date_of_birth, phone, bio
            "#,
            true,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context(format!(
            "Failed to update email verification status for user id {} in database",
            id
        ))?;
        Ok(user)
    }

    // Create an email verification token
    pub async fn create_email_verification_token(
        &self,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> anyhow::Result<EmailVerificationToken> {
        let now = Utc::now();

        // Delete any existing tokens for this user
        sqlx::query!(
            r#"
            DELETE FROM email_verification_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete existing email verification tokens")?;

        // Create a new token
        let token = sqlx::query_as!(
            EmailVerificationToken,
            r#"
            INSERT INTO email_verification_tokens (user_id, token, expires_at, created)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, token, expires_at, created
            "#,
            user_id,
            token,
            expires_at,
            now
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create email verification token")?;

        Ok(token)
    }

    // Get an email verification token by token string
    pub async fn get_email_verification_token(
        &self,
        token: &str,
    ) -> anyhow::Result<Option<EmailVerificationToken>> {
        let token = sqlx::query_as!(
            EmailVerificationToken,
            r#"
            SELECT id, user_id, token, expires_at, created
            FROM email_verification_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch email verification token")?;

        Ok(token)
    }

    // Delete an email verification token
    pub async fn delete_email_verification_token(&self, token_id: Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM email_verification_tokens
            WHERE id = $1
            "#,
            token_id
        )
        .execute(&self.pool)
        .await
        .context(format!(
            "Failed to delete email verification token with id {}",
            token_id
        ))?;
        Ok(())
    }

    // Update a user last login
    pub async fn update_last_login(&self, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login = $1
            WHERE id = $2
            "#,
            now as DateTime<Utc>,
            id
        )
        .execute(&self.pool)
        .await
        .context(format!(
            "Failed to update last login for user id {} in database",
            id
        ))?;
        Ok(())
    }
}
