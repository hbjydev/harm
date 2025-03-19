use std::{path::PathBuf, process::exit};

use anyhow::{Error, Result};
use auth::ControllerAuthenticator;
use axum::Router;
use clap::Parser;
use harm_config::Config;
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/etc/harm/config.yaml")]
    config_file: PathBuf,
}

mod api;
mod auth;
mod db;
mod state;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_init();
    let args = Args::parse();

    let cfg = get_config_or_exit(args.config_file);
    let db = db::Db::new(cfg.control_plane.clone())
        .await
        .map_err(|e| Error::msg(format!("failed to open database: {}", e)))?;

    db.ping()
        .await
        .map_err(|e| Error::msg(format!("failed to ping database: {}", e)))?;

    let authn = ControllerAuthenticator::new(db.clone());
    let app_state = AppState::new(cfg.clone(), db, authn);

    let router = Router::new()
        .route("/v0/servers", axum::routing::get(api::v0::servers::list_servers))
        .with_state(app_state);

    let listener = TcpListener::bind(cfg.control_plane.listen_addr).await?;

    Ok(axum::serve(listener, router).await?)
}

fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn get_config_or_exit(path: PathBuf) -> Config {
    match Config::read_from_file(path) {
        Err(err) => {
            println!("Failed to read config: {}", err);
            exit(1);
        }
        Ok(cfg) => cfg,
    }
}
