use crate::utils::password::hash_password;
use anyhow::Context;
use sqlx::PgPool;
use tracing;

// User domain
use crate::domain::user::model::Role;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::service::UserService;

// Settings
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub user_service: UserService,
}

impl AppState {
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        let pool = init_db(settings).await?;

        // Initialize repositories and services
        let user_repository = UserRepository::new(pool.clone());
        let user_service = UserService::new(user_repository);

        Ok(Self {
            settings: settings.clone(),
            user_service,
        })
    }
}

// Function to initialize the PostgreSQL database connection pool
async fn init_db(settings: &Settings) -> anyhow::Result<PgPool> {
    let url = settings
        .database
        .url
        .clone()
        .context("Database URL is not set")?;
    let pool = PgPool::connect(&url).await?;
    // Run migrations if the setting is explicitly true, default to false if not set
    if settings.database.migrate_on_startup.unwrap_or(false) {
        sqlx::migrate!("./database/migrations").run(&pool).await?;
        // Seed admin user after migrations
        seed_admin_user(&pool).await?;
    }
    Ok(pool)
}

// Function to seed the admin user if it doesn't exist
async fn seed_admin_user(pool: &PgPool) -> anyhow::Result<()> {
    let user_repository = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repository);

    // Check if admin user already exists
    if user_service.get_user_by_email("admin").await.is_ok() {
        tracing::info!("Admin user already exists, skipping seed");
        return Ok(());
    }

    // Hash the password
    let password_hash = hash_password("admin")?;

    // Create admin user
    match user_service
        .create_user(
            "Admin".to_string(),
            "admin".to_string(),
            "admin".to_string(),
            password_hash,
            Role::Admin,
        )
        .await
    {
        Ok(_) => tracing::info!("Admin user seeded successfully"),
        Err(e) => tracing::warn!("Failed to seed admin user: {}", e),
    }

    Ok(())
}
