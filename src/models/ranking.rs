use serde::{Serialize, Deserialize};

use super::league::*;
use super::badge_urls::BadgeUrls;
use super::location::Location;
use super::paging::Paging;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    pub attack_wins: u32,
    pub clan: Option<PlayerRankingClan>,
    pub defense_wins: u32,
    pub name: String,
    pub rank: u64,
    pub tag: String,
    pub trophies: u16,

    pub league: Option<League>,
    pub exp_level: Option<u16>,
    pub previous_rank: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRankingList {
    pub items: Vec<PlayerRanking>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRankingClan {
    pub badge_urls: BadgeUrls,
    pub name: String,
    pub tag: String,
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
pub struct ClanRankingList {
    pub items: Vec<ClanRanking>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBuilderBaseRanking {
    pub builder_base_league: BuilderLeague,
    pub clan: PlayerRankingClan,
    pub tag: String,
    pub name: String,
    pub exp_level: u16,
    pub rank: u64,
    pub previous_rank: u64,
    pub builder_base_trophies: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBuilderBaseRankingList {
    pub items: Vec<PlayerBuilderBaseRanking>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanBuilderBaseRanking {
    pub badge_urls: BadgeUrls,
    pub clan_builder_base_points: u32,
    pub clan_level: u8,
    pub location: Location,
    pub members: u8,
    pub name: String,
    pub previous_rank: u32,
    pub rank: u32,
    pub tag: String,
    pub clan_points: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanBuilderBaseRankingList {
    pub items: Vec<ClanBuilderBaseRanking>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRanking {
    pub clan_capital_points: u32,
    pub clan_points: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanCapitalRankingList {
    pub items: Vec<ClanCapitalRanking>,
    pub paging: Paging,
}
