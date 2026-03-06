use thiserror::Error;

pub fn init() -> Result<(), TelemetryError> {
    tracing_subscriber::fmt()
        .try_init()
        .map_err(TelemetryError::Init)
}

#[derive(Debug, Error)]
pub enum TelemetryError {
    #[error("failed to initialize telemetry: {0}")]
    Init(#[source] Box<dyn std::error::Error + Send + Sync>),
}