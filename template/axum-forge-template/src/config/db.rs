use thiserror::Error;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, DatabaseConfigError> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:55432/axum_forge".to_string()
        });

        let max_connections = match std::env::var("DATABASE_MAX_CONNECTIONS") {
            Ok(value) => value
                .parse::<u32>()
                .map_err(|source| DatabaseConfigError::InvalidMaxConnections { value, source })?,
            Err(_) => 5,
        };

        Ok(Self {
            url,
            max_connections,
        })
    }
}

#[derive(Debug, Error)]
pub enum DatabaseConfigError {
    #[error("invalid DATABASE_MAX_CONNECTIONS value '{value}': {source}")]
    InvalidMaxConnections {
        value: String,
        #[source]
        source: std::num::ParseIntError,
    },
}
