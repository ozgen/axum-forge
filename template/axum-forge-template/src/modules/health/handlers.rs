use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
}

pub async fn healthz() -> Json<StatusResponse> {
    Json(StatusResponse { status: "ok" })
}

pub async fn readyz() -> Json<StatusResponse> {
    Json(StatusResponse { status: "ready" })
}