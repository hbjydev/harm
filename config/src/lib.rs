use std::{
    fs, net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use utils::detect_reforger_path;

mod utils;
pub mod paths;

const LISTEN_ADDR_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub control_plane: ControlPlaneConfig,
    pub harmlet: HarmletConfig,
}

impl Config {
    pub fn read_from_file(config_path: PathBuf) -> Result<Config> {
        let str_config = fs::read_to_string(config_path)?;
        Ok(serde_yaml::from_str(&str_config)?)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct ControlPlaneConfig {
    /// The address to start the server on.
    pub listen_addr: SocketAddr,

    /// The location of the libsql service to run.
    /// This can be a local filesystem path, server address, or :memory: for
    /// development.
    pub db_addr: String,

    /// If using a remote libsql service (and if a token is required), the
    /// authentication token to connect to the service with.
    pub db_token: Option<String>,
}

impl Default for ControlPlaneConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::new(LISTEN_ADDR_DEFAULT, 15443),
            db_addr: String::from(":memory:"),
            db_token: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct HarmletConfig {
    /// The address to start the server on.
    pub listen_addr: SocketAddr,

    /// The path to the directory containing the Arma Reforger server binary.
    pub reforger_path: Option<PathBuf>,
}

impl Default for HarmletConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::new(LISTEN_ADDR_DEFAULT, 38503),
            reforger_path: detect_reforger_path(),
        }
    }
}
