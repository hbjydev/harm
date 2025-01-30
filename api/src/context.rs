use sea_orm::DatabaseConnection;

pub struct ServerCtx {
    pub db: DatabaseConnection,
    pub reforger_path: String,
}
