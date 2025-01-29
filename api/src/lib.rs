use std::net::{Ipv4Addr, SocketAddr};

use dropshot::{ApiDescription, ConfigDropshot, ConfigLogging, ServerBuilder};

struct ServerCtx {}

pub async fn start(port: u16) -> Result<(), String> {
    let config_dropshot = ConfigDropshot {
        bind_address: SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), port)),
        ..Default::default()
    };

    let config_log = ConfigLogging::StderrTerminal {
        level: dropshot::ConfigLoggingLevel::Info
    };
    let log = config_log
        .to_logger("harm_server")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    let ctx = ServerCtx {};

    let api = ApiDescription::<ServerCtx>::new();

    let server = ServerBuilder::new(api, ctx, log)
        .config(config_dropshot)
        .start()
        .map_err(|error| format!("failed to start server: {}", error))?;

    server.await
}
