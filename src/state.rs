use crate::utils::password::hash_password;
use anyhow::Context;
use sqlx::PgPool;

// User domain
use crate::domain::user::model::Role;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::service::UserService;

// Note domain
use crate::domain::note::repository::NoteRepository;
use crate::domain::note::service::NoteService;

// Auth domain
use crate::domain::auth::service::AuthService;

// Settings
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub user_service: UserService,
    pub note_service: NoteService,
    pub auth_service: AuthService,
}

impl AppState {
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        let pool = init_db(settings).await?;

        // Initialize repositories and services
        let user_repository = UserRepository::new(pool.clone());
        let user_service = UserService::new(user_repository.clone());

        let note_repository = NoteRepository::new(pool.clone());
        let note_service = NoteService::new(note_repository);

        // Initialize auth service with JWT configuration
        let jwt_secret = settings
            .jwt
            .secret
            .clone()
            .unwrap_or_else(|| "default_secret_change_in_production".to_string());
        let access_token_expiration_hours = settings.jwt.access_token_expiration_hours.unwrap_or(1);
        let refresh_token_expiration_hours =
            settings.jwt.refresh_token_expiration_hours.unwrap_or(720);
        let auth_service = AuthService::new(
            user_repository,
            &jwt_secret,
            access_token_expiration_hours,
            refresh_token_expiration_hours,
        );

        Ok(Self {
            settings: settings.clone(),
            user_service,
            note_service,
            auth_service,
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
    if user_service
        .get_user_by_email("admin@example.com")
        .await
        .is_ok()
    {
        tracing::info!("Admin user already exists, skipping seed");
        return Ok(());
    }

    // Hash the password
    let password_hash = hash_password("admin")?;

    // Create admin user
    let create_input = crate::domain::user::service::CreateUserInput {
        display_name: Some("Admin".to_string()),
        username: Some("admin".to_string()),
        email: Some("admin@example.com".to_string()),
        password_hash,
        role: Role::Admin,
    };

    match user_service.create_user(create_input).await {
        Ok(_) => tracing::info!("Admin user seeded successfully"),
        Err(e) => tracing::warn!("Failed to seed admin user: {}", e),
    }

    Ok(())
}
