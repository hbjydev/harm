use std::fs;
use std::io::Write;
use std::path::PathBuf;

use dropshot::{endpoint, ClientErrorStatusCode, HttpError, HttpResponseOk, RequestContext};
use dropshot::{EmptyScanParams, PaginationParams, Path, Query, ResultsPage, TypedBody, WhichPage};
use entity::config::{self, Entity as ConfigEntity, GameConfig, ServerConfig};
use entity::config::{ModConfig, Model as ConfigModel};
use schemars::JsonSchema;
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

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
        Self { id: value.id }
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
        WhichPage::First(..) => ConfigEntity::find()
            .limit(limit)
            .all(db)
            .await
            .map_err(|error| HttpError::for_internal_error(error.to_string())),

        WhichPage::Next(ServerPage { id }) => ConfigEntity::find()
            .limit(limit)
            .filter(config::Column::Id.gt(id.clone()))
            .order_by_asc(config::Column::Id)
            .all(db)
            .await
            .map_err(|error| HttpError::for_internal_error(error.to_string())),
    }?;

    Ok(HttpResponseOk(ResultsPage::new(
        configs,
        &EmptyScanParams {},
        |p: &ConfigModel, _| ServerPage::from(p),
    )?))
}

#[derive(JsonSchema, Deserialize)]
struct GetServerPath {
    /// The ID of the server to fetch data for.
    id: Uuid,
}

#[endpoint(
    method = GET,
    path = "/servers/{id}",
)]
pub async fn get_server(
    rqctx: RequestContext<ServerCtx>,
    path: Path<GetServerPath>,
) -> Result<HttpResponseOk<ConfigModel>, HttpError> {
    let db = &rqctx.context().db;
    let path = path.into_inner();

    let config = ConfigEntity::find()
        .filter(Expr::col(config::Column::Id).eq(path.id))
        .one(db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    if let Some(cfg) = config {
        Ok(HttpResponseOk(cfg))
    } else {
        Err(HttpError::for_not_found(
            Some("NO_SUCH_SERVER".to_string()),
            "No server with that ID was found.".to_string(),
        ))
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
struct CreateServerBody {
    title: String,
}

#[endpoint(
    method = POST,
    path = "/servers",
)]
pub async fn create_server(
    rqctx: RequestContext<ServerCtx>,
    rqbody: TypedBody<CreateServerBody>,
) -> Result<HttpResponseOk<ConfigModel>, HttpError> {
    let body = rqbody.into_inner();
    let db = &rqctx.context().db;

    let insert = ConfigEntity::insert(config::ActiveModel {
        id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
        title: sea_orm::ActiveValue::Set(body.title.clone()),
        config: sea_orm::ActiveValue::Set(ServerConfig {
            game: GameConfig {
                name: body.title,
                ..Default::default()
            },
            ..Default::default()
        }),
    })
    .exec_with_returning(db)
    .await
    .map_err(|error| {
        HttpError::for_internal_error(format!("failed to insert server: {}", error))
    })?;

    Ok(HttpResponseOk(insert))
}

#[derive(JsonSchema, Deserialize, Serialize)]
struct AddModResponse {
    success: bool,
}

#[derive(JsonSchema, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AddModRequest {
    mod_id: String,
    name: Option<String>,
}

#[endpoint(
    method = POST,
    path = "/servers/{id}/mods"
)]
pub async fn add_mod(
    rqctx: RequestContext<ServerCtx>,
    path: Path<GetServerPath>,
    body: TypedBody<AddModRequest>,
) -> Result<HttpResponseOk<AddModResponse>, HttpError> {
    let db = &rqctx.context().db;
    let path = path.into_inner();

    let config = ConfigEntity::find()
        .filter(Expr::col(config::Column::Id).eq(path.id))
        .one(db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    if let Some(mut cfg) = config {
        let mod_body = body.into_inner();

        let existing_ids: Vec<String> = cfg
            .config
            .game
            .mods
            .iter()
            .map(|e| e.mod_id.clone())
            .collect();
        if existing_ids.contains(&mod_body.mod_id.clone()) {
            return Err(HttpError::for_client_error(
                Some("MOD_ALREADY_ADDED".to_string()),
                ClientErrorStatusCode::BAD_REQUEST,
                "A mod with that ID already exists on this server's configuration!".to_string(),
            ));
        }

        let mod_block = ModConfig {
            mod_id: mod_body.mod_id,
            name: match mod_body.name {
                Some(name) => name,
                None => String::new(),
            },
            required: true,
        };

        cfg.config.game.mods.push(mod_block);

        ConfigEntity::update(config::ActiveModel {
            id: sea_orm::ActiveValue::Unchanged(cfg.id),
            title: sea_orm::ActiveValue::Unchanged(cfg.title),
            config: sea_orm::ActiveValue::Set(cfg.config),
        })
        .exec(db)
        .await
        .map_err(|e| HttpError::for_internal_error(format!("failed to update config: {}", e)))?;

        return Ok(HttpResponseOk(AddModResponse { success: true }));
    }

    Err(HttpError::for_not_found(
        Some("NO_SUCH_SERVER".to_string()),
        "No server with that ID was found.".to_string(),
    ))
}

#[derive(JsonSchema, Deserialize, Serialize)]
struct ListModsResponse {
    mods: Vec<ModConfig>,
}

#[endpoint(
    method = GET,
    path = "/servers/{id}/mods"
)]
pub async fn list_mods(
    rqctx: RequestContext<ServerCtx>,
    path: Path<GetServerPath>,
) -> Result<HttpResponseOk<ListModsResponse>, HttpError> {
    let db = &rqctx.context().db;
    let path = path.into_inner();

    let config = ConfigEntity::find()
        .filter(Expr::col(config::Column::Id).eq(path.id))
        .one(db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    if let Some(cfg) = config {
        return Ok(HttpResponseOk(ListModsResponse { mods: cfg.config.game.mods }));
    }

    Err(HttpError::for_not_found(
        Some("NO_SUCH_SERVER".to_string()),
        "No server with that ID was found.".to_string(),
    ))
}

#[endpoint(
    method = POST,
    path = "/servers/{id}/start"
)]
pub async fn start_server(
    rqctx: RequestContext<ServerCtx>,
    path: Path<GetServerPath>,
) -> Result<HttpResponseOk<AddModResponse>, HttpError> {
    let db = &rqctx.context().db;
    let path = path.into_inner();

    let config = ConfigEntity::find()
        .filter(Expr::col(config::Column::Id).eq(path.id))
        .one(db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    if let Some(cfg) = config {
        let path = PathBuf::from(&rqctx.context().reforger_path);
        let parent_path = path.parent().unwrap();
        let config_path = parent_path.join(format!("{}.json", cfg.id.clone()));
        let config_str = serde_json::to_string(&cfg.config).map_err(|e| HttpError::for_internal_error(format!("failed to spawn server: {}", e)))?;

        let mut file = fs::File::create(config_path.clone()).unwrap();
        file.write_all(config_str.as_bytes()).unwrap();

        let mut child = tokio::process::Command::new(path.clone())
            .current_dir(parent_path)
            .arg("-maxFPS")
            .arg("60")
            .arg("-config")
            .arg(config_path.to_str().unwrap())
            .spawn()
            .expect("failed to spawn");

        return Ok(HttpResponseOk(AddModResponse{ success: true }));
    }

    Err(HttpError::for_not_found(
        Some("NO_SUCH_SERVER".to_string()),
        "No server with that ID was found.".to_string(),
    ))
}

#[derive(JsonSchema, Deserialize)]
struct ModPath {
    /// The ID of the server to fetch data for.
    id: Uuid,

    /// The ID of the mod to fetch data for.
    mod_id: String,
}

#[endpoint(
    method = DELETE,
    path = "/servers/{id}/mods/{mod_id}"
)]
pub async fn delete_mod(
    rqctx: RequestContext<ServerCtx>,
    path: Path<ModPath>,
) -> Result<HttpResponseOk<AddModResponse>, HttpError> {
    let db = &rqctx.context().db;
    let path = path.into_inner();

    let config = ConfigEntity::find()
        .filter(Expr::col(config::Column::Id).eq(path.id))
        .one(db)
        .await
        .map_err(|error| HttpError::for_internal_error(error.to_string()))?;

    if let Some(mut cfg) = config {
        let existing_ids: Vec<String> = cfg
            .config
            .game
            .mods
            .iter()
            .map(|e| e.mod_id.clone())
            .collect();
        if !existing_ids.contains(&path.mod_id.clone()) {
            return Err(HttpError::for_client_error(
                Some("MOD_NOT_ADDED".to_string()),
                ClientErrorStatusCode::BAD_REQUEST,
                "No mod with that ID exists on this server's configuration!".to_string(),
            ));
        }

        let idx = cfg.config.game.mods.iter().position(|x| x.mod_id == path.mod_id.clone()).unwrap();
        cfg.config.game.mods.remove(idx);

        ConfigEntity::update(config::ActiveModel {
            id: sea_orm::ActiveValue::Unchanged(cfg.id),
            title: sea_orm::ActiveValue::Unchanged(cfg.title),
            config: sea_orm::ActiveValue::Set(cfg.config),
        })
        .exec(db)
        .await
        .map_err(|e| HttpError::for_internal_error(format!("failed to update config: {}", e)))?;

        return Ok(HttpResponseOk(AddModResponse { success: true }));
    }

    Err(HttpError::for_not_found(
        Some("NO_SUCH_SERVER".to_string()),
        "No server with that ID was found.".to_string(),
    ))
}
