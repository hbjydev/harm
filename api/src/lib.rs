use std::net::{Ipv4Addr, SocketAddr};

use context::ServerCtx;
use dropshot::{ApiDescription, ConfigDropshot, ConfigLogging, ServerBuilder};
use harm_migration::MigratorTrait;
use harm_pm::manager::ProcessManager;

mod apis;
mod context;
mod db;

pub async fn start(port: u16, database_url: String, reforger_path: String) -> Result<(), String> {
    let config_dropshot = ConfigDropshot {
        bind_address: SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), port)),
        ..Default::default()
    };

    let config_log = ConfigLogging::StderrTerminal {
        level: dropshot::ConfigLoggingLevel::Info,
    };
    let log = config_log
        .to_logger("harm_server")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    let db_conn = db::conn(database_url)
        .await
        .map_err(|error| format!("failed to open db conn: {}", error))?;

    harm_migration::Migrator::up(&db_conn, None)
        .await
        .map_err(|error| format!("failed to migrate db: {}", error))?;

    let process_manager = ProcessManager::new(reforger_path, log.clone());

    let ctx = ServerCtx {
        db: db_conn,
        process_manager,
    };

    let mut api = ApiDescription::<ServerCtx>::new();
    api.register(apis::server::list_servers).unwrap();
    api.register(apis::server::get_server).unwrap();
    api.register(apis::server::create_server).unwrap();
    api.register(apis::server::start_server).unwrap();
    api.register(apis::server::stop_server).unwrap();
    api.register(apis::server::add_mod).unwrap();
    api.register(apis::server::list_mods).unwrap();
    api.register(apis::server::delete_mod).unwrap();

    let server = ServerBuilder::new(api, ctx, log)
        .config(config_dropshot)
        .start()
        .map_err(|error| format!("failed to start server: {}", error))?;

    server.await
}
