use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub type Result<T> = anyhow::Result<T, ControllerError>;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Chrono error occurred.")]
    ChronoError(#[from] chrono::ParseError),

    #[error("UUID error occurred.")]
    UuidError(#[from] uuid::Error),

    #[error("Serialization error occurred.")]
    JsonError(#[from] serde_json::Error),

    #[error("Database error occurred.")]
    DBError(#[from] libsql::Error),

    #[error("Database deserialize error occurred.")]
    DBDeError(#[from] serde::de::value::Error),

    #[error("Something went wrong.")]
    UnexpectedError(#[from] anyhow::Error)
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> axum::response::Response {
        let err_str = self.to_string();
        tracing::error!(
            %self,
            "error handling request"
        );

        match self {
            Self::ChronoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ControllerErrorRepr {
                    error: err_str,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            Self::UuidError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ControllerErrorRepr {
                    error: err_str,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            Self::JsonError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ControllerErrorRepr {
                    error: err_str,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            Self::DBError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ControllerErrorRepr {
                    error: err_str,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            Self::DBDeError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ControllerErrorRepr {
                    error: err_str,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            Self::UnexpectedError(_err) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ControllerErrorRepr {
                        error: err_str,
                        reason: None,
                    })
                ).into_response()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ControllerErrorRepr {
    pub error: String,
    pub reason: Option<String>,
}
