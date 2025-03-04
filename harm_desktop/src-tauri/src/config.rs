use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

pub fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let harm_config_dir = config_dir.join("harm");
    if !fs::exists(&harm_config_dir).unwrap_or(false) {
        fs::create_dir(&harm_config_dir).expect("Could not create HARM config directory.");
    }
    harm_config_dir
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub reforger_path: Option<String>,
    pub api_port: u16,
}

impl AppConfig {
    pub fn read() -> Self {
        let config_dir = config_path();
        let config_path = config_dir.join("config.json");

        if let Ok(config_str) = fs::read_to_string(config_path) {
            serde_json::from_str(&config_str).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = config_path();
        let config_path = config_dir.join("config.json");
        let config_str = serde_json::to_string(self)?;
        fs::write(config_path, config_str.as_bytes())?;
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            reforger_path: None,
            api_port: 10625,
        }
    }
}
