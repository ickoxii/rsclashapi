use serde::{Serialize, Deserialize};

use crate::models::enums::clan::{BattleModifier, WarResult};
use crate::models::paging::Paging;
use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLog {
    pub items: Option<Vec<ClanWarLogEntry>>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLogEntry {
    pub result: Option<WarResult>,
    pub end_time: String,
    pub team_size: u8,
    pub attacks_per_member: Option<u8>,
    pub battle_modifier: Option<BattleModifier>,
    pub clan: Option<WarClan>,
    pub opponent: Option<OpponentWarClan>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: Option<BadgeUrls>,
    pub clan_level: u8,
    pub attacks: u8,
    pub stars: u16,
    pub destruction_percentage: f32,
    pub exp_earned: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentWarClan {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: BadgeUrls,
    pub clan_level: u8,
    pub stars: u16,
    pub destruction_percentage: f32,
}
