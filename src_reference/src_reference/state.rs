use crate::domain::v1;
use crate::settings::Settings;
use crate::utils::email::{EmailConfig, EmailService};
use crate::utils::jwt::JwtUtil;

use anyhow::Context;
use arc_swap::ArcSwap;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

// Define the AppStateInner struct
pub struct AppStateInner {
    pub settings: ArcSwap<Settings>,
    pub jwt_util: Arc<JwtUtil>,
    pub email_service: Option<Arc<EmailService>>,
    pub services_v1: Arc<v1::Services>,
}

// Define the AppState struct to hold the AppStateInner
#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

// Implement Deref to allow accessing AppStateInner fields directly from AppState
impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implementation of the AppState struct
impl AppState {
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        // Initialize postgres database connection pool
        let pool = init_db(&settings).await?;

        // Initialize JWT utility
        let jwt_secret = settings
            .jwt
            .secret
            .clone()
            .context("JWT secret is not set")?;
        let jwt_expiration = settings.jwt.expiration_hours.unwrap_or(24);
        let jwt_util = Arc::new(JwtUtil::new(&jwt_secret, jwt_expiration));

        // Initialize email service if configured
        let email_service = if let (
            Some(smtp_host),
            Some(smtp_port),
            Some(smtp_username),
            Some(smtp_password),
            Some(from_email),
            Some(from_name),
            Some(verification_url_base),
        ) = (
            settings.email.smtp_host.clone(),
            settings.email.smtp_port,
            settings.email.smtp_username.clone(),
            settings.email.smtp_password.clone(),
            settings.email.from_email.clone(),
            settings.email.from_name.clone(),
            settings.email.verification_url_base.clone(),
        ) {
            let email_config = EmailConfig {
                smtp_host,
                smtp_port,
                smtp_username,
                smtp_password,
                from_email,
                from_name,
                verification_url_base,
            };
            Some(Arc::new(EmailService::new(email_config)))
        } else {
            None
        };

        // Initialize services
        let mut services_v1 = v1::Services::new(pool);

        // Add email service to services if available
        if let Some(email_svc) = &email_service {
            services_v1 = services_v1.with_email_service(email_svc.clone());
        }

        // Return the application state
        Ok(Self(Arc::new(AppStateInner {
            settings: ArcSwap::new(Arc::new(settings.clone())),
            jwt_util,
            email_service,
            services_v1: Arc::new(services_v1),
        })))
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
