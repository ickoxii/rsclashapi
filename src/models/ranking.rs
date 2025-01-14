use serde::{Serialize, Deserialize};

use super::league::League;
use super::badge_urls::BadgeUrls;
use super::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    pub league: League,
    pub clan: PlayerRankingClan,
    pub attack_wins: u32,
    pub defense_wins: u32,
    pub tag: String,
    pub name: String,
    pub exp_level: u16,
    pub rank: u64,
    pub previous_rank: u64,
    pub trophies: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRankingList(pub Vec<PlayerRanking>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRankingClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanRanking {
    pub clan_points: u32,
    pub clan_level: u8,
    pub location: Location,
    pub members: u8,
    pub tag: String,
    pub name: String,
    pub rank: u64,
    pub previous_rank: u64,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanRankingList(pub Vec<ClanRanking>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBuilderBaseRanking {
    pub builder_base_league: League,
    pub clan: PlayerRankingClan,
    pub tag: String,
    pub name: String,
    pub exp_level: u16,
    pub rank: u64,
    pub previous_rank: u64,
    pub builder_base_trophies: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBuilderBaseRankingList(pub Vec<PlayerBuilderBaseRanking>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanBuilderBaseRanking {
    pub clan_builder_base_points: u32,
    pub clan_points: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanBuilderBaseRankingList(pub Vec<ClanBuilderBaseRanking>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRanking {
    pub clan_capital_points: u32,
    pub clan_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanCapitalRankingList(pub Vec<ClanCapitalRanking>);
