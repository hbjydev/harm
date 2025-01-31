use std::collections::BTreeMap;

use harm_pm::Action;
use sea_orm::DatabaseConnection;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

pub struct ServerCtx {
    pub db: DatabaseConnection,
    pub reforger_path: String,
    pub server_channels: Mutex<BTreeMap<String, UnboundedSender<Action>>>,
}
