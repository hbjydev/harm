use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub type Result<T> = anyhow::Result<T, ControllerError>;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
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
