use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// JWT claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub exp: usize,    // Expiration time (as UTC timestamp)
    pub iat: usize,    // Issued at (as UTC timestamp)
    pub email: String, // User email
}

// JWT utility struct
pub struct JwtUtil {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_hours: i64,
}

// Implementation of JwtUtil
impl JwtUtil {
    // Constructor for JwtUtil
    pub fn new(secret: &str, expiration_hours: i64) -> Self {
        // Create encoding and decoding keys from the secret
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        // Return a new instance of JwtUtil
        Self {
            encoding_key,
            decoding_key,
            expiration_hours,
        }
    }

    // Method to generate a JWT token
    pub fn generate_token(&self, user_id: Uuid, email: &str) -> anyhow::Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize,
            email: email.to_string(),
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .context("Failed to generate token")?;
        Ok(token)
    }

    // Method to verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .context("Failed to decode token")?;

        Ok(token_data.claims)
    }
}
