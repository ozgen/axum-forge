mod app;
mod config;
mod error;
mod modules;
mod state;
mod telemetry;

use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    telemetry::init()?;

    let config = config::Config::from_env()?;
    let app = app::build_app(&config).await?;

    let address = config.app.address();
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(StartupError::Bind)?;

    tracing::info!("server listening on {}", address);

    axum::serve(listener, app)
        .await
        .map_err(StartupError::Serve)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum StartupError {
    #[error("telemetry initialization failed: {0}")]
    Telemetry(#[from] telemetry::TelemetryError),

    #[error("configuration loading failed: {0}")]
    Config(#[from] config::ConfigError),

    #[error("application build failed: {0}")]
    App(#[from] app::BuildAppError),

    #[error("failed to bind TCP listener: {0}")]
    Bind(#[source] std::io::Error),

    #[error("server failed: {0}")]
    Serve(#[source] std::io::Error),
}
