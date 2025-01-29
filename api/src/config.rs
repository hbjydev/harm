use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use entity::config::Entity as ConfigEntity;
use entity::config::Model as ConfigModel;
use sea_orm::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::context::ServerCtx;

#[derive(Deserialize, Serialize, JsonSchema)]
struct ListServersResponse {
    pub servers: Vec<ConfigModel>,
}

#[endpoint(
    method = GET,
    path = "/servers",
)]
pub async fn list_servers(
    rqctx: RequestContext<ServerCtx>,
) -> Result<HttpResponseOk<ListServersResponse>, HttpError> {
    let configs: Vec<ConfigModel> = ConfigEntity::find()
        .all(&rqctx.context().db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    let res = ListServersResponse { servers: configs };

    Ok(HttpResponseOk(res))
}
