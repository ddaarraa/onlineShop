use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::env;

pub struct EnvConfig {
    pub jwt_secret: Option<String>,
    pub database_url: Option<String>,
}

impl EnvConfig {
    // Function to get the JWT secret key
    pub fn get_jwt_secret(&self) -> Result<String, String> {
        self.jwt_secret
            .clone()
            .ok_or("JWT_SECRET is not set in the environment".to_string())
    }
}

lazy_static! {
    static ref ENV_CONFIG: Arc<Mutex<EnvConfig>> = Arc::new(Mutex::new(EnvConfig {
        jwt_secret: env::var("JWT_SECRET").ok(),
        database_url: env::var("DATABASE_URL").ok(),
    }));
}

// Function to access the singleton configuration
pub fn get_env_config() -> Arc<Mutex<EnvConfig>> {
    Arc::clone(&ENV_CONFIG)
}

pub fn get_jwt_secret_from_config() -> Result<String, String> {
    let env_config = get_env_config();
    let config_guard = env_config
        .lock()
        .map_err(|_| "Failed to acquire lock on env_config".to_string())?;
    config_guard.get_jwt_secret()
}
