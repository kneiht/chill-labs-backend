use crate::entities::Role;
use anyhow::Result;
use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum ExpiresIn {
    OneHour,
    SevenDays,
}

impl ExpiresIn {
    fn to_duration(&self) -> Duration {
        match self {
            ExpiresIn::OneHour => Duration::from_secs(3600),
            ExpiresIn::SevenDays => Duration::from_secs(604800),
        }
    }
}

#[async_trait]
pub trait JsonWebToken {
    async fn sign(&self, payload: JwtPayload, expires_in: ExpiresIn) -> Result<String>;
    async fn verify(&self, token: &str) -> Result<JwtPayload>;
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            encoding_key,
            decoding_key,
        }
    }
}

#[async_trait]
impl JsonWebToken for JwtService {
    async fn sign(&self, mut payload: JwtPayload, expires_in: ExpiresIn) -> Result<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        payload.exp = Some(now + expires_in.to_duration().as_secs());
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &payload, &self.encoding_key)?;
        Ok(token)
    }

    async fn verify(&self, token: &str) -> Result<JwtPayload> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<JwtPayload>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

#[async_trait]
impl JsonWebToken for std::sync::Arc<JwtService> {
    async fn sign(&self, payload: JwtPayload, expires_in: ExpiresIn) -> Result<String> {
        (**self).sign(payload, expires_in).await
    }

    async fn verify(&self, token: &str) -> Result<JwtPayload> {
        (**self).verify(token).await
    }
}
