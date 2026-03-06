pub mod app;
pub mod db;

pub use app::{AppConfig, AppConfigError};
pub use db::{DatabaseConfig, DatabaseConfigError};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub db: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            app: AppConfig::from_env()?,
            db: DatabaseConfig::from_env()?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load app config: {0}")]
    App(#[from] AppConfigError),

    #[error("failed to load database config: {0}")]
    Database(#[from] DatabaseConfigError),
}
