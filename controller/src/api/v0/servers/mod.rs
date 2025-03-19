use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::{db::{models::Server, Db}, errors::Result};

#[derive(Serialize)]
struct ListServersResponse {
    pub servers: Vec<Server>,
    pub count: usize,
}

pub async fn list_servers(
    State(db): State<Db>,
) -> Result<impl IntoResponse> {
    let servers = Server::find_all(&db).await?;
    Ok((StatusCode::OK, Json(ListServersResponse {
        servers: servers.clone(),
        count: servers.len(),
    })))
}
