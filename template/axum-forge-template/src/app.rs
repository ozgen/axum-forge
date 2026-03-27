use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    modules::{
        health,
        items::{self},
    },
    state::AppState,
};

pub async fn build_app(config: &Config) -> Result<Router, BuildAppError> {
    let db = PgPoolOptions::new()
        .max_connections(config.db.max_connections)
        .connect(&config.db.url)
        .await
        .map_err(BuildAppError::DatabaseConnection)?;

    // Migrate db always
    sqlx::migrate!("./migrations").run(&db).await?;

    let state = AppState::new(db);

    let app = Router::new()
        .route("/healthz", get(health::handlers::healthz))
        .route("/readyz", get(health::handlers::readyz))
        .route("/api/v1/items", get(items::handlers::list_items))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

#[derive(Debug, Error)]
pub enum BuildAppError {
    #[error("failed to connect to database: {0}")]
    DatabaseConnection(#[source] sqlx::Error),

    #[error("failed to run migrations: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
