use anyhow::Result;
use harm_config::ControlPlaneConfig;
use libsql::{Builder, Connection};

pub(super) mod models;

#[derive(Clone)]
pub struct Db(Connection);

impl Db {
    pub async fn new(cfg: ControlPlaneConfig) -> Result<Db> {
        if cfg.db_addr.starts_with("libsql://") {
            if let Some(token) = cfg.db_token {
                Ok(Db(Builder::new_remote(cfg.db_addr, token).build().await?.connect()?))
            } else {
                Ok(Db(Builder::new_remote(cfg.db_addr, String::from(""))
                    .build()
                    .await?.connect()?))
            }
        } else {
            Ok(Db(Builder::new_local(cfg.db_addr).build().await?.connect()?))
        }
    }

    pub async fn ping(&self) -> Result<()> {
        self.conn().query("select 1; select 1;", ()).await?;
        Ok(())
    }

    pub fn conn(&self) -> Connection {
        self.0.clone()
    }
}
