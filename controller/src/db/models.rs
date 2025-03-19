use crate::errors::Result;
use chrono::Utc;
use harm_schema::harm::servers::v0::ServerConfig;
use libsql::{de, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;

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

    pub async fn insert(&self, db: &Db) -> Result<()> {
        let conn = db.conn();
        let stmt = conn.prepare(
            "insert into servers (id, title, config, created_at) values (?, ?, ?, ?)",
        ).await?;


        Ok(())
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
