pub mod error;
pub mod healthcheck;
pub mod response;
pub mod user;

// Transformer trait for converting inputs to model types with validation
pub trait Transformer<T> {
    fn transform(self) -> Result<T, crate::domain::error::AppError>;
}

// Implement Transformer for the model type itself (identity transformation)
impl<T> Transformer<T> for T {
    fn transform(self) -> Result<T, crate::domain::error::AppError> {
        Ok(self)
    }
}

// Implement Transformer<String> for &str for ergonomic string handling
impl Transformer<String> for &str {
    fn transform(self) -> Result<String, crate::domain::error::AppError> {
        Ok(self.to_string())
    }
}
