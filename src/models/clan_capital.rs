use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::badge_urls::BadgeUrls;
use crate::models::paging::Paging;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: Option<u8>,
    pub districts: Option<Vec<ClanDistrictData>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanDistrictData {
    pub name: String, // JsonLocalizedName
    pub id: CapitalDistrictId,
    pub district_hall_level: u8,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum CapitalDistrictId {
    CapitalPeak = 70_000_000,
    BarbarianCamp = 70_000_001,
    WizardValley = 70_000_002,
    BalloonLagoon = 70_000_003,
    BuildersWorkshop = 70_000_004,
    DragonCliffs = 70_000_005,
    GolemQuarry = 70_000_006,
    SkeletonPark = 70_000_007,
    GoblinMines = 70_000_008,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeason {
    pub state: String, // Get actual state enums
    pub start_time: String,
    pub end_time: String,
    pub capital_total_loot: u32,
    pub raids_completed: u16,
    pub total_attacks: u16,
    pub enemy_districts_destroyed: u16,
    pub offensive_reward: u16,
    pub defensive_reward: u16,
    pub members: Option<ClanCapitalRaidSeasonMemberList>,
    pub attack_log: ClanCapitalRaidSeasonAttackLog,
    pub defense_log: ClanCapitalRaidSeasonDefenseLog,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasons {
    pub items: Vec<ClanCapitalRaidSeason>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonAttackLogEntry {
    pub defender: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: u16,
    pub district_count: u8,
    pub districts_destroyed: u16,
    pub districts: ClanCapitalRaidSeasonDistrictList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasonAttackLog(pub Vec<ClanCapitalRaidSeasonAttackLogEntry>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonDefenseLogEntry {
    pub attacker: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: u16,
    pub district_count: u8,
    pub districts_destroyed: u16,
    pub districts: ClanCapitalRaidSeasonDistrictList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasonDefenseLog(pub Vec<ClanCapitalRaidSeasonDefenseLogEntry>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonMember {
    pub tag: String,
    pub name: String,
    pub attacks: u8,
    pub attack_limit: u8,
    pub bonus_attack_limit: u8,
    pub capital_resources_looted: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasonMemberList(pub Vec<ClanCapitalRaidSeasonMember>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonClanInfo {
    pub tag: String,
    pub name: String,
    pub level: u8,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonDistrict {
    pub id: CapitalDistrictId,
    pub name: String, // JsonLocalizedName
    pub district_hall_level: u8,
    pub destruction_percent: u8,
    pub stars: u8,
    pub attack_count: u8,
    pub total_looted: u16,
    pub attacks: Option<Vec<ClanCapitalRaidSeasonAttack>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasonDistrictList(pub Vec<ClanCapitalRaidSeasonDistrict>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonAttack {
    pub attacker: ClanCapitalRaidSeasonAttacker,
    pub destruction_percent: u8,
    pub stars: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanCapitalRaidSeasonAttackList(pub Vec<ClanCapitalRaidSeasonAttack>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonAttacker {
    pub tag: String,
    pub name: String,
}
