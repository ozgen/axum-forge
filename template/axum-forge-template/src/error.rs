use std::borrow::Cow;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AppError {
    pub status: StatusCode,
    pub code: Cow<'static, str>,
    pub message: Cow<'static, str>,
    pub details: Option<ErrorDetails>,
}

impl AppError {
    pub fn new(
        status: StatusCode,
        code: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: ErrorDetails) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_optional_details(mut self, details: Option<ErrorDetails>) -> Self {
        self.details = details;
        self
    }

    pub fn bad_request(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    pub fn unauthorized(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    pub fn forbidden(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "forbidden", message)
    }

    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    pub fn conflict(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::CONFLICT, "conflict", message)
    }

    pub fn validation(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "validation_error",
            message,
        )
    }

    pub fn unprocessable_entity(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            message,
        )
    }

    pub fn service_unavailable(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "service_unavailable",
            message,
        )
    }

    pub fn internal(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    pub fn internal_from_error<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        tracing::error!("internal error: {error}");
        Self::internal("an unexpected error occurred")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status;

        let body = Json(ErrorResponse {
            error: ErrorBody {
                code: self.code.into_owned(),
                message: self.message.into_owned(),
                details: self.details,
            },
        });

        (status, body).into_response()
    }
}

pub trait ToAppError {
    fn to_app_error(self) -> AppError;
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Option<ErrorDetails>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ErrorDetails {
    Text(String),
    Validation(ValidationDetails),
    Fields(Vec<FieldError>),
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationDetails {
    pub fields: Vec<FieldError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
