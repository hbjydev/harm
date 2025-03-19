use std::{env, path::PathBuf};

use tokio::sync::OnceCell;

static CONFIG_DIR: OnceCell<PathBuf> = OnceCell::const_new();

#[cfg(target_os = "windows")]
mod platform {
    use std::path::PathBuf;

    pub fn get_data_dir() -> PathBuf {
        PathBuf::from("C:\\ProgramData\\harm\\data")
    }

    pub fn get_config_dir() -> PathBuf {
        PathBuf::from("C:\\ProgramData\\harm\\config")
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use std::path::PathBuf;

    pub fn get_data_dir() -> PathBuf {
        PathBuf::from("/var/lib/harm")
    }

    pub fn get_config_dir() -> PathBuf {
        PathBuf::from("/etc/harm")
    }
}

pub async fn get_config_dir_path() -> PathBuf {
    CONFIG_DIR.get_or_init(|| async {
        match env::var("HARM_CONFIG_DIR") {
            Err(_) => platform::get_config_dir(),
            Ok(val) => val.parse().unwrap(),
        }
    }).await.to_path_buf()
}

pub async fn get_data_dir_path() -> PathBuf {
    CONFIG_DIR.get_or_init(|| async {
        match env::var("HARM_DATA_DIR") {
            Err(_) => platform::get_data_dir(),
            Ok(val) => val.parse().unwrap(),
        }
    }).await.to_path_buf()
}

pub async fn get_database_path() -> PathBuf {
    get_data_dir_path().await.join("harm.db")
}

pub async fn get_key_dir_path() -> PathBuf {
    get_data_dir_path().await.join("key")
}

pub async fn get_private_key_path() -> PathBuf {
    get_key_dir_path().await.join("private").with_extension("pem")
}

pub async fn get_public_key_path() -> PathBuf {
    get_key_dir_path().await.join("public").with_extension("pem")
}

pub async fn get_cacert_path() -> PathBuf {
    get_key_dir_path().await.join("cacert").with_extension("pem")
}
