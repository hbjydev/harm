use sea_orm::DatabaseConnection;

pub struct ServerCtx {
    pub db: DatabaseConnection,
}
