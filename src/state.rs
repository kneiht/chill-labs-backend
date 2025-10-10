use crate::settings::Settings;
use anyhow::Context;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub db: PgPool,
}

impl AppState {
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        let pool = init_db(settings).await?;

        Ok(Self {
            settings: settings.clone(),
            db: pool,
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
    }
    Ok(pool)
}
