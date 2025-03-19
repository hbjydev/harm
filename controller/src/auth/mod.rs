use anyhow::Result;
use axum::http::{HeaderMap, header::AUTHORIZATION};
use uuid::Uuid;

use crate::db::{
    Db,
    models::{TokenRow, TokenType},
};

/// A resolved identity, mapping a token to an identity type and identifier.
#[derive(Debug, serde::Deserialize)]
pub enum Identity {
    /// A node identity, providing the node's ID according to the control
    /// plane.
    Node(Uuid),

    /// A user identity, providing the user's ID according to the control
    /// plane.
    User(Uuid),

    /// An unauthenticated identity.
    None,
}

impl From<TokenRow> for Identity {
    fn from(value: TokenRow) -> Self {
        match value.token_type {
            TokenType::Node => Self::Node(value.identity_id),
            TokenType::User => Self::User(value.identity_id),
        }
    }
}

#[derive(Clone)]
pub struct ControllerAuthenticator {
    db: Db,
}

impl ControllerAuthenticator {
    pub fn new(db: Db) -> ControllerAuthenticator {
        ControllerAuthenticator { db }
    }

    pub async fn authenticate(
        &self,
        headers: HeaderMap,
        token_type: TokenType,
    ) -> Result<Identity> {
        Ok(match headers.get(AUTHORIZATION) {
            Some(header_val) => {
                let token_val = header_val.to_str()?.to_string();
                let found_token = TokenRow::find(self.db.conn(), token_type, token_val).await?;
                match found_token {
                    Some(row) => row.into(),
                    None => Identity::None,
                }
            }
            None => Identity::None,
        })
    }
}
