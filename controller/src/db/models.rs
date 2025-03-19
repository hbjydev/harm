use std::str::FromStr;

use crate::errors::Result;
use chrono::{DateTime, Utc};
use harm_schema::harm::servers::v0::ServerConfig;
use libsql::{Connection, Row, de, params};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: Uuid,
    pub title: String,
    pub created_at: chrono::DateTime<Utc>,
    pub config: harm_schema::harm::servers::v0::ServerConfig,
}

impl Server {
    pub fn new(title: String, config: ServerConfig) -> Server {
        Server {
            id: Uuid::new_v4(),
            title,
            created_at: Utc::now(),
            config,
        }
    }

    pub async fn find_all(db: &Db) -> Result<Vec<Self>> {
        let conn = db.conn();

        let mut stmt = conn
            .prepare("select id, title, created_at, config from servers")
            .await?;

        let mut rows = stmt.query(()).await?;
        let mut servers = Vec::new();

        loop {
            match rows.next().await? {
                Some(row) => servers.push(Self::from_row(&row)?),
                None => break,
            }
        }

        Ok(servers)
    }

    pub async fn find_by_id(db: &Db, id: Uuid) -> Result<Self> {
        let conn = db.conn();
        let mut stmt = conn
            .prepare("select id, title, created_at, config from servers where id = ?")
            .await?;

        let row = stmt.query_row(params!(id.to_string(),)).await?;

        Ok(Self::from_row(&row)?)
    }

    pub async fn insert(&self, db: &Db) -> Result<()> {
        let conn = db.conn();
        let mut stmt = conn
            .prepare("insert into servers (id, title, config, created_at) values (?, ?, ?, ?)")
            .await?;

        let conf_str: String = self.config.clone().try_into()?;

        stmt.execute(params!(
            self.id.to_string(),
            self.title.clone(),
            conf_str,
            self.created_at.to_string(),
        ))
        .await?;

        Ok(())
    }

    fn from_row(row: &Row) -> Result<Self> {
        let id = Uuid::parse_str(row.get_str(0)?)?;
        let title = row.get(1)?;
        let created_at = DateTime::<Utc>::from_str(row.get_str(1)?)?;

        let config = serde_json::from_str(row.get_str(3)?)?;

        Ok(Server {
            id,
            title,
            created_at,
            config,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TokenType {
    Node,
    User,
}

impl Into<String> for TokenType {
    fn into(self) -> String {
        match self {
            Self::Node => String::from("node"),
            Self::User => String::from("user"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenRow {
    pub id: Uuid,
    pub token_type: TokenType,
    pub identity_id: Uuid,
    pub issued_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
    pub active: bool, // used to disable tokens before their expiry
}

impl TokenRow {
    pub async fn find(
        conn: Connection,
        token_type: TokenType,
        token: String,
    ) -> Result<Option<TokenRow>> {
        let mut stmt = conn
            .prepare("select * from auth_tokens where token_type = ? and token = ? limit 1")
            .await?;

        let token: TokenRow = match stmt.query_row(&[token_type.into(), token]).await {
            Ok(row) => de::from_row(&row)?,
            Err(err) => match err {
                libsql::Error::QueryReturnedNoRows => {
                    return Ok(None);
                }
                err => {
                    return Err(err.into());
                }
            },
        };

        Ok(Some(token))
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= chrono::Utc::now()
    }

    pub fn is_valid(&self) -> bool {
        self.active && !self.is_expired()
    }
}
