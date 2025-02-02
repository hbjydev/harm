use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub reforger_path: Option<String>,
    pub api_port: u16,
}

impl AppConfig {
    pub fn read() -> Self {
        let config_dir = dirs::config_dir().unwrap();
        let config_path = config_dir.join("config.json");

        if let Ok(config_str) = fs::read_to_string(config_path) {
            let config_res: serde_json::Result<Self> = serde_json::from_str(&config_str);
            match config_res {
                Ok(config) => config,
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().unwrap();
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
