use crate::entities::users;
use crate::utils::password::hash_password;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait, QueryFilter,
    Set,
};
use std::time::Duration;

// Auth domain
use crate::domain::user::service::UserService;

// Settings
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub db: DatabaseConnection,
    pub user_service: UserService,
}

impl AppState {
    /// Initialize the application state.
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        // Initialize database connection
        let db = init_db(settings).await?;

        // Seed admin user
        seed_admin_user(&db, settings).await?;

        // Initialize JWT secret
        let jwt_secret = settings.jwt.secret.clone();

        // Initialize JWT expiration hours
        let access_token_expiration_hours = settings.jwt.access_token_expiration_hours;
        let refresh_token_expiration_hours = settings.jwt.refresh_token_expiration_hours;

        // Initialize user service
        let user_service = UserService::new(
            db.clone(),
            &jwt_secret,
            access_token_expiration_hours,
            refresh_token_expiration_hours,
        );

        // Initialize state
        Ok(Self {
            settings: settings.clone(),
            db,
            user_service,
        })
    }
}

/// Initialize the Database connection
async fn init_db(settings: &Settings) -> anyhow::Result<DatabaseConnection> {
    let url = settings.database.url.clone();

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(tracing::log::LevelFilter::Info);

    let db = sea_orm::Database::connect(opt).await?;

    // Run migrations if the setting is explicitly true, default to false if not set
    // Note: SeaORM migration is different, usually via sea-orm-cli or programmatic.
    // For now, we skip automatic migration here or use sqlx if needed, but mixing is tricky.
    // Let's assume migrations are run externally or we add sea-orm-migration crate later.

    Ok(db)
}

async fn seed_admin_user(db: &DatabaseConnection, settings: &Settings) -> anyhow::Result<()> {
    // Get admin email and password from settings (required fields)
    let email = &settings.admin.email;
    let password = &settings.admin.password;

    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await?;

    if existing_user.is_none() {
        tracing::info!("Seeding admin user: {}", email);
        let password_hash = hash_password(password)?;
        let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

        let user = users::ActiveModel {
            id: Set(uuid::Uuid::now_v7()),
            username: Set(Some("admin".to_string())),
            email: Set(Some(email.clone())),
            display_name: Set(Some("Admin User".to_string())),
            password_hash: Set(password_hash),
            role: Set("admin".to_string()),
            status: Set("active".to_string()),
            created: Set(now),
            updated: Set(now),
        };

        user.insert(db).await?;
        tracing::info!("Admin user seeded successfully");
    } else {
        tracing::info!("Admin user already exists");
    }

    Ok(())
}
