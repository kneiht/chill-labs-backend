use config::{Config, Environment};
use serde::Deserialize;

// Define the Database struct to hold the database configuration
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[allow(unused)]
pub struct Database {
    pub url: String,
    pub migrate_on_startup: bool,
}

// Define the Logging struct to hold the logging configuration
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Logging {
    pub log_level: String, // e.g., "debug", "info", "warn", "error", "trace" or RUST_LOG directives
    pub log_format: String, // e.g., "pretty", "json"
    pub log_file: Option<String>, // Optional - can be None if logging to stdout
}

// Define the ConfigInfo struct to hold the configuration information
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ConfigInfo {
    pub env_prefix: String, // e.g., "APP"
}

// Define an enum for Server Environment
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")] // Allows "dev" or "prod" strings to map to Dev or Prod variants
pub enum ServerEnv {
    Dev,
    Prod,
}

// Define the Server struct to hold the server configuration
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub env: ServerEnv, // e.g., "dev", "prod"
    pub host: String,
    pub port: u16,
}

// Define the Jwt struct to hold the JWT configuration
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Jwt {
    pub secret: String,
    pub access_token_expiration_hours: i64,
    pub refresh_token_expiration_hours: i64,
}

// Define the Admin struct to hold the admin user configuration
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Admin {
    pub email: String,
    pub password: String,
}

// Define the Settings struct to hold all the configuration settings
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub database: Database,
    pub logging: Logging,
    pub config: ConfigInfo,
    pub server: Server,
    pub jwt: Jwt,
    pub admin: Admin,
}

// Implement the Settings struct
impl Settings {
    pub fn new(env_prefix: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings: Settings = s.try_deserialize()?;

        // The deserialization process itself will now validate the ServerEnv.
        // If `server.env` is specified in the configuration with an invalid value (not "dev" or "prod"),
        // `s.try_deserialize()` would have returned an error.
        Ok(settings)
    }
}
