use dropshot::{EmptyScanParams, PaginationParams, Query, ResultsPage, WhichPage};
use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use entity::config::{self, Entity as ConfigEntity};
use entity::config::Model as ConfigModel;
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::context::ServerCtx;

#[derive(Deserialize, Serialize, JsonSchema)]
struct ListServersResponse {
    pub servers: Vec<ConfigModel>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ServerPage {
    pub id: Uuid,
}

impl From<&ConfigModel> for ServerPage {
    fn from(value: &ConfigModel) -> Self {
        Self {
            id: value.id,
        }
    }
}

#[endpoint(
    method = GET,
    path = "/servers",
)]
pub async fn list_servers(
    rqctx: RequestContext<ServerCtx>,
    query: Query<PaginationParams<EmptyScanParams, ServerPage>>,
) -> Result<HttpResponseOk<ResultsPage<ConfigModel>>, HttpError> {
    let pag_params = query.into_inner();
    let limit = rqctx.page_limit(&pag_params)?.get() as u64;
    let db = &rqctx.context().db;

    let configs = match &pag_params.page {
        WhichPage::First(..) => {
            ConfigEntity::find()
                .limit(limit)
                .all(db)
                .await
                .map_err(|error| HttpError::for_internal_error(error.to_string()))
        }

        WhichPage::Next(ServerPage { id, .. }) => {
            ConfigEntity::find()
                .limit(limit)
                .filter(config::Column::Id.gt(id.clone()))
                .order_by_asc(config::Column::Id)
                .all(db)
                .await
                .map_err(|error| HttpError::for_internal_error(error.to_string()))
        }
    }?;

    Ok(HttpResponseOk(ResultsPage::new(
        configs,
        &EmptyScanParams {},
        |p: &ConfigModel, _| ServerPage::from(p),
    )?))
}
