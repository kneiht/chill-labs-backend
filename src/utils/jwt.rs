use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration time (as UTC timestamp)
    pub iat: usize,         // Issued at (as UTC timestamp)
    pub email: String,      // User email
    pub token_type: TokenType, // Token type (access or refresh)
}

// JWT utility struct
#[derive(Clone)]
pub struct JwtUtil {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiration_hours: i64,
    refresh_token_expiration_hours: i64,
}

impl JwtUtil {
    pub fn new(
        secret: &str,
        access_token_expiration_hours: i64,
        refresh_token_expiration_hours: i64,
    ) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            encoding_key,
            decoding_key,
            access_token_expiration_hours,
            refresh_token_expiration_hours,
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> anyhow::Result<String> {
        self.generate_token_internal(
            user_id,
            email,
            TokenType::Access,
            self.access_token_expiration_hours,
        )
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, email: &str) -> anyhow::Result<String> {
        self.generate_token_internal(
            user_id,
            email,
            TokenType::Refresh,
            self.refresh_token_expiration_hours,
        )
    }

    fn generate_token_internal(
        &self,
        user_id: Uuid,
        email: &str,
        token_type: TokenType,
        expiration_hours: i64,
    ) -> anyhow::Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize,
            email: email.to_string(),
            token_type,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .context("Failed to generate token")?;
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .context("Failed to decode token")?;

        Ok(token_data.claims)
    }
}
