pub mod auth;
pub mod error;
pub mod healthcheck;
pub mod note;
pub mod response;
pub mod user;

use crate::domain::error::AppError;
use validator::Validate as DeriveValidate;

// Transformer trait for converting inputs to model types with validation
pub trait Transformer<T> {
    fn transform(self) -> Result<T, crate::domain::error::AppError>;
}

// Blanket implementation for types that implement Validate
impl<T> Transformer<T> for T
where
    T: DeriveValidate,
{
    fn transform(self) -> Result<T, AppError> {
        self.validate()
            .map_err(|e| AppError::validation(&e.to_string()))?;
        Ok(self)
    }
}

// Implement Transformer<String> for &str for ergonomic string handling
impl Transformer<String> for &str {
    fn transform(self) -> Result<String, AppError> {
        Ok(self.to_string())
    }
}
