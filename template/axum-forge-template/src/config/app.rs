use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppConfigError> {
        let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = match std::env::var("APP_PORT") {
            Ok(value) => value
                .parse::<u16>()
                .map_err(|source| AppConfigError::InvalidPort { value, source })?,
            Err(_) => 3001,
        };

        Ok(Self { host, port })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("invalid APP_PORT value '{value}': {source}")]
    InvalidPort {
        value: String,
        #[source]
        source: std::num::ParseIntError,
    },
}
