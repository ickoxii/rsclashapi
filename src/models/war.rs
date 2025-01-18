use serde::{Serialize, Deserialize};

use crate::models::enums::clan::WarState;
use crate::models::enums::clan::BattleModifier;
use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWar {
    pub state: WarState,
    pub team_size: u8,
    pub attacks_per_member: Option<u8>,
    pub preparation_start_time: String,
    pub start_time: String,
    pub end_time: String,
    pub clan: WarClan,
    pub opponent: WarClan,
    pub battle_modifier: Option<BattleModifier>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    pub map_position: u8,
    pub townhall_level: u8,
    pub opponent_attacks: u8,
    pub best_opponent_attack: Option<ClanWarAttack>,
    pub attacks: Option<Vec<ClanWarAttack>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarAttack {
    pub order: u8,
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: u8,
    pub destruction_percentage: u8,
    pub duration: u16,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct ClanWarAttackList(pub Vec<ClanWarAttack>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: Option<BadgeUrls>,
    pub clan_level: u8,
    pub attacks: Option<u8>,
    pub stars: u16,
    pub destruction_percentage: f32,
    pub members: Vec<ClanWarMember>,
}
