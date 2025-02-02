use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
extern crate schemars;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "config")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: uuid::Uuid,

    pub title: String,

    #[sea_orm(json)]
    pub config: harm_schemas::ServerConfig,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
