use std::net::{Ipv4Addr, SocketAddr};

use dropshot::{endpoint, ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpError, HttpResponseOk, RequestContext, ServerBuilder};

struct Context {}

#[endpoint(
    method = GET,
    path = "/example",
)]
async fn example_endpoint(
    rqctx: RequestContext<Context>
) -> Result<HttpResponseOk<bool>, HttpError> {
    Ok(dropshot::HttpResponseOk(true))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let port = std::env::args()
        .nth(1)
        .map(|p| p.parse::<u16>())
        .transpose()
        .map_err(|e| format!("failed to parse \"port\" argument: {}", e))?
        .unwrap_or(0);

    let config_dropshot = ConfigDropshot {
        bind_address: SocketAddr::from((Ipv4Addr::LOCALHOST, port)),
        ..Default::default()
    };

    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = config_logging
        .to_logger("example-basic")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    let mut api = ApiDescription::new();
    api.register(example_endpoint).unwrap();

    let server = ServerBuilder::new(api, Context {}, log)
        .config(config_dropshot)
        .start()
        .map_err(|e| format!("failed to create server: {}", e))?;

    server.await
}
