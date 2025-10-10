use crate::domain::v1::user::{UserRepository, UserService};
use crate::utils::email::EmailService;
use sqlx::PgPool;
use std::sync::Arc;

// Holds all services
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<UserService>,
}

// Implementation of the Services struct
impl Services {
    pub fn new(pool: PgPool) -> Self {
        // Initialize repositories
        let user_repository = UserRepository::new(pool.clone());

        // Initialize services
        let user_service = Arc::new(UserService::new(user_repository));

        // Return the Services struct
        Self { user_service }
    }

    // Update the user service with an email service
    pub fn with_email_service(mut self, email_service: Arc<EmailService>) -> Self {
        // Create a new user service with the email service
        let user_service = Arc::new(
            UserService::new(UserRepository::new(
                // We need to extract the pool from the post repository
                // This is a bit of a hack, but it works
                Arc::clone(&self.user_service).repository.pool.clone(),
            ))
            .with_email_service(email_service),
        );

        // Replace the user service
        self.user_service = user_service;
        self
    }
}
