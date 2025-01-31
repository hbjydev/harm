use std::path::PathBuf;

use anyhow::Result;
use entity::config::ServerConfig;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[derive(Debug, Default)]
pub enum Action {
    Stop,

    #[default]
    Nothing,
}

pub fn get_server_ch() -> (UnboundedSender<Action>, UnboundedReceiver<Action>) {
    mpsc::unbounded_channel::<Action>()
}

pub async fn run_server(
    reforger_path: String,
    server_id: String,
    config: ServerConfig,
    mut rx: UnboundedReceiver<Action>,
) -> Result<()> {
    let path = PathBuf::from(reforger_path);
    let parent_path = path.parent().unwrap();
    let config_path = parent_path.join(format!("{}.json", server_id.clone()));
    let config_str = serde_json::to_string(&config)?;
    let mut file = fs::File::create(config_path.clone()).await?;
    file.write_all(config_str.as_bytes()).await?;

    let mut child = tokio::process::Command::new(path.clone())
        .current_dir(parent_path)
        .arg("-maxFPS")
        .arg("60")
        .arg("-config")
        .arg(config_path.to_str().unwrap())
        .spawn()
        .expect("failed to spawn");

    tokio::select! {
        _ = child.wait() => {}
        msg = rx.recv() => match msg.unwrap() {
            Action::Stop => { child.kill().await? }
            Action::Nothing => {}
        }
    }

    Ok(())
}
