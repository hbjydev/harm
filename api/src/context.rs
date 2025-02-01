use harm_pm::manager::ProcessManager;
use sea_orm::DatabaseConnection;

pub struct ServerCtx {
    pub process_manager: ProcessManager,
    pub db: DatabaseConnection,
}
