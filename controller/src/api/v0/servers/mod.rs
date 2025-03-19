use axum::{http::StatusCode, response::IntoResponse};

use crate::errors::Result;

pub async fn list_servers() -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, "hello, world!"))
}
