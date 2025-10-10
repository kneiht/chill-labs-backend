use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
// Helper method to hash passwords
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Manually handle the error conversion since password_hash::Error doesn't implement std::error::Error
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Password hashing error: {}", e))?
        .to_string();

    Ok(password_hash)
}

// Helper method to verify passwords
pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<()> {
    // Manually handle the error conversion
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow!("Password hash parsing error: {}", e))?;
    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow!("Invalid username or password"))
}
