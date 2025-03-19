use axum::extract::FromRef;
use harm_config::Config;

use crate::{auth::ControllerAuthenticator, db::Db};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    pub authn: ControllerAuthenticator,
    pub db: Db,
}

impl AppState {
    pub fn new(
        config: Config,
        db: Db,
        authn: ControllerAuthenticator
    ) -> Self {
        Self { config, db, authn }
    }
}
