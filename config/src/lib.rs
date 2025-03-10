use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use utils::detect_reforger_path;

mod utils;

const LISTEN_ADDR_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub control_plane: ControlPlaneConfig,
    pub harmlet: HarmletConfig,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ControlPlaneConfig {
    /// The address to start the server on.
    pub listen_addr: SocketAddr,
}

impl Default for ControlPlaneConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::new(LISTEN_ADDR_DEFAULT, 15443),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
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
