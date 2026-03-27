use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

pub async fn healthz() -> Json<StatusResponse> {
    Json(StatusResponse { status: "ok" })
}

pub async fn readyz(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(json!({
        "status": "ready"
    })))
}
