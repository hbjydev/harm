use std::{collections::HashMap, path::PathBuf, process::Stdio, sync::Arc};

use anyhow::{Error, Result};
use harm_entity::config::ServerConfig;
use slog::{debug, error, info, Logger};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Child,
    sync::Mutex,
};
use uuid::Uuid;

#[derive(Debug)]
pub enum ServerState {
    Running,
    Stopped,
}

#[derive(Debug)]
pub struct Server {
    pub id: Uuid,
    pub process: Option<Child>,
    pub state: ServerState,
}

/// ProcessManager is a simple process manager built to track and interact with
/// multiple servers (datatypes defined by the harm_entity crate).
pub struct ProcessManager {
    pub arma_reforger_path: String,
    logger: Logger,
    servers: Arc<Mutex<HashMap<Uuid, Server>>>,
}

impl ProcessManager {
    pub fn new(arma_reforger_path: String, logger: Logger) -> Self {
        Self {
            arma_reforger_path,
            logger,
            servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Gets three paths for a given server and returns them in a tuple in the
    /// following order:
    /// - The Arma Reforger Path
    /// - The Arma Reforger _directory_ path
    /// - The path for this server's config file
    fn get_paths(&self, id: Uuid) -> (PathBuf, PathBuf, PathBuf) {
        let path = PathBuf::from(self.arma_reforger_path.clone());
        let parent_path = path.parent().unwrap().to_path_buf();
        let config_path = parent_path.join(format!("{}.json", id.clone()));

        debug!(
            self.logger,
            "AR: {:?}, RP: {:?}, CP: {:?}", path, parent_path, config_path
        );

        (path, parent_path, config_path)
    }

    /// Writes a server's configuration (via the ServerConfig struct) to the
    /// relevant file on the host's filesystem.
    pub async fn write_config(&self, id: Uuid, config: ServerConfig) -> Result<()> {
        let (_, _, config_path) = self.get_paths(id);
        let config_str = serde_json::to_string(&config)?;
        let mut file = fs::File::create(config_path.clone()).await?;
        file.write_all(config_str.as_bytes()).await?;

        Ok(())
    }

    /// Starts a server using tokio::process and returns the Child handle back
    /// to the caller.
    async fn _start_server(&self, id: Uuid, config: ServerConfig) -> Result<Child> {
        let (exec_path, parent_path, config_path) = self.get_paths(id);
        self.write_config(id, config).await?;

        info!(
            self.logger,
            "Spawning AR process for server {}",
            id.to_string()
        );

        let child = tokio::process::Command::new(exec_path.clone())
            .current_dir(parent_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("-maxFPS")
            .arg("60")
            .arg("-config")
            .arg(config_path.to_str().unwrap())
            .spawn()?;

        Ok(child)
    }

    /// Starts a new server by UUID, if it is not running. If it is running,
    /// this function will return an error.
    pub async fn start_server(&self, id: Uuid, config: ServerConfig) -> Result<()> {
        let mut servers = self.servers.lock().await;
        if let Some(server) = servers.get_mut(&id) {
            if server.process.is_some() {
                return Err(Error::msg("That server is already running."));
            }

            let child = self._start_server(id, config.clone()).await?;
            server.process = Some(child);
        } else {
            let child = self._start_server(id, config.clone()).await?;
            let server = Server {
                id,
                process: Some(child),
                state: ServerState::Running,
            };
            servers.insert(id, server);
        }

        Ok(())
    }

    /// Stops a server by UUID, if it is running.
    pub async fn stop_server(&self, id: Uuid) -> Result<()> {
        let mut servers = self.servers.lock().await;
        if let Some(server) = servers.get_mut(&id) {
            if let Some(mut process) = server.process.take() {
                info!(
                    self.logger,
                    "Stopping AR process for server {}",
                    id.to_string()
                );
                process.kill().await?;
                server.state = ServerState::Stopped;
            } else {
                error!(
                    self.logger,
                    "AR process for server {} already dead!",
                    id.to_string()
                );
                return Err(Error::msg("That server is already stopped!"));
            }
        } else {
            error!(
                self.logger,
                "AR process for server {} already dead!",
                id.to_string()
            );
            return Err(Error::msg("That server was not started!"));
        }

        Ok(())
    }

    /// Returns the logs from a server, provided a UUID of the server logs are
    /// needed for. Return value is (stdout, stderr)
    pub async fn get_logs(&self, id: Uuid) -> Result<(String, String)> {
        let mut servers = self.servers.lock().await;
        if let Some(server) = servers.get_mut(&id) {
            let mut o_stdout = String::new();
            let mut o_stderr = String::new();
            if let Some(mut process) = server.process.take() {
                if let Some(mut stdout) = process.stdout.take() {
                    stdout.read_to_string(&mut o_stdout).await?;
                }
                if let Some(mut stderr) = process.stderr.take() {
                    stderr.read_to_string(&mut o_stderr).await?;
                }
            }
            return Ok((o_stdout, o_stderr));
        }

        Err(Error::msg("No server registered by that ID."))
    }
}
