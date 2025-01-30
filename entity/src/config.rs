use std::vec;

use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, JsonSchema, Serialize, Deserialize)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: uuid::Uuid,

    pub title: String,

    #[sea_orm(json)]
    pub config: ServerConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct A2SConfig {
    pub address: String,
    pub port: u16,
}

impl Default for A2SConfig {
    fn default() -> Self {
        Self {
            address: String::from("0.0.0.0"),
            port: 17777,
        }
    }
}

#[derive(
    Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema,
)]
pub enum RconPermission {
    /// The admin can perform any command.
    #[serde(rename = "admin")]
    Admin,

    /// The monitor can only perform commands which do not change the server's state.
    #[default]
    #[serde(rename = "monitor")]
    Monitor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RconConfig {
    pub address: String,
    pub port: u16,
    pub password: String,
    pub max_clients: u16,
    pub permission: RconPermission,
    pub blacklist: Vec<String>,
    pub whitelist: Vec<String>,
}

impl Default for RconConfig {
    fn default() -> Self {
        Self {
            address: String::from("0.0.0.0"),
            port: 19999,
            password: String::from("changeme_withoutspaces"),
            max_clients: 16,
            permission: RconPermission::default(),
            blacklist: Vec::new(),
            whitelist: Vec::new(),
        }
    }
}

#[derive(
    Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema,
)]
pub enum GamePlatform {
    /// PC
    #[serde(rename = "PLATFORM_PC")]
    #[default]
    PC,

    /// Xbox Console
    #[serde(rename = "PLATFORM_XBL")]
    XBL,

    /// PlayStation Console
    #[serde(rename = "PLATFORM_PSN")]
    PSN,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GameProperties {
    pub server_max_view_distance: u16,
    pub server_min_grass_distance: u16,
    pub fast_validation: bool,
    pub network_view_distance: u16,
    pub battleye: bool,
    pub disable_third_person: bool,
    pub von_disable_ui: bool,
    pub von_disable_direct_speech_ui: bool,
    pub von_can_transmit_cross_faction: bool,
    pub mission_header: Option<Json>,
}

impl Default for GameProperties {
    fn default() -> Self {
        Self {
            server_max_view_distance: 1600,
            server_min_grass_distance: 0,
            network_view_distance: 1500,
            disable_third_person: false,
            fast_validation: true,
            battleye: true,
            von_disable_ui: false,
            von_disable_direct_speech_ui: false,
            von_can_transmit_cross_faction: false,
            mission_header: None,
        }
    }
}

#[derive(
    Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct ModConfig {
    pub mod_id: String,
    pub name: String,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    pub name: String,
    pub password: Option<String>,
    pub password_admin: String,
    pub admins: Vec<String>,
    pub scenario_id: String,
    pub max_players: u16,
    pub visible: bool,
    pub cross_platform: bool,
    pub supported_platforms: Vec<GamePlatform>,
    pub game_properties: GameProperties,
    pub mods_required_by_default: bool,
    pub mods: Vec<ModConfig>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            password: None,
            password_admin: String::new(),
            admins: Vec::new(),
            scenario_id: String::from("{59AD59368755F41A}Missions/21_GM_Eden.conf"),
            max_players: 64,
            visible: true,
            cross_platform: false,
            supported_platforms: vec![GamePlatform::PC],
            game_properties: GameProperties::default(),
            mods_required_by_default: true,
            mods: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JoinQueueConfig {
    pub max_size: i16,
}

impl Default for JoinQueueConfig {
    fn default() -> Self {
        Self { max_size: 50 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConfig {
    pub lobby_player_synchronize: bool,
    pub disable_crash_reporter: bool,
    pub disable_navmesh_streaming: Option<Vec<String>>,
    pub disable_server_shutdown: bool,
    pub disable_ai: bool,
    pub player_save_time: i16,
    pub ai_limit: i16,
    pub slot_reservation_timeout: i16,
    pub join_queue: JoinQueueConfig,
}

impl Default for OperatingConfig {
    fn default() -> Self {
        Self {
            lobby_player_synchronize: true,
            disable_crash_reporter: false,
            disable_navmesh_streaming: None,
            disable_server_shutdown: false,
            disable_ai: false,
            player_save_time: 120,
            ai_limit: -1,
            slot_reservation_timeout: 60,
            join_queue: JoinQueueConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub bind_address: String,
    pub bind_port: Option<u16>,
    pub public_address: Option<String>,
    pub public_port: u16,
    pub a2s: A2SConfig,
    pub rcon: RconConfig,
    pub game: GameConfig,
    pub operating: OperatingConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: String::from("0.0.0.0"),
            bind_port: None,
            public_address: None,
            public_port: 2001,
            a2s: A2SConfig::default(),
            rcon: RconConfig::default(),
            game: GameConfig::default(),
            operating: OperatingConfig::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
