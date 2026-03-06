use thiserror::Error;

use crate::error::{AppError, ToAppError};

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("{entity} not found")]
    NotFound { entity: &'static str },

    #[error("{entity} with {field} '{value}' was not found")]
    NotFoundBy {
        entity: &'static str,
        field: &'static str,
        value: String,
    },

    #[error("{entity} already exists")]
    AlreadyExists { entity: &'static str },

    #[error("{entity} with {field} '{value}' already exists")]
    AlreadyExistsBy {
        entity: &'static str,
        field: &'static str,
        value: String,
    },

    #[error("invalid {entity}: {message}")]
    Validation {
        entity: &'static str,
        message: String,
    },

    #[error("{entity} conflict: {message}")]
    Conflict {
        entity: &'static str,
        message: String,
    },
}

impl ToAppError for EntityError {
    fn to_app_error(self) -> AppError {
        match self {
            Self::NotFound { entity } => AppError::not_found(format!("{entity} not found")),
            Self::NotFoundBy {
                entity,
                field,
                value,
            } => AppError::not_found(format!("{entity} with {field} '{value}' was not found")),
            Self::AlreadyExists { entity } => {
                AppError::conflict(format!("{entity} already exists"))
            }
            Self::AlreadyExistsBy {
                entity,
                field,
                value,
            } => AppError::conflict(format!("{entity} with {field} '{value}' already exists")),
            Self::Validation { entity, message } => {
                AppError::validation(format!("invalid {entity}: {message}"))
            }
            Self::Conflict { entity, message } => {
                AppError::conflict(format!("{entity} conflict: {message}"))
            }
        }
    }
}
