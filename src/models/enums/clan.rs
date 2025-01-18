use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Role {
    #[serde(rename = "notMember")]
    NotMember,

    #[serde(rename = "member")]
    Member,

    #[serde(rename = "admin")]
    Elder,

    #[serde(rename = "coLeader")]
    Coleader,

    #[serde(rename = "leader")]
    Leader,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WarLeagueState {
    GroupNotFound,
    NotInWar,
    Preparation,
    War,
    Ended,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WarState {
    ClanNotFound,
    AccessDenied,
    NotInWar,
    InMatchmaking,
    EnterWar,
    Matched,
    Preparation,
    War,
    InWar,
    Ended,
    WarEnded, // When getting a clan war league war??
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WarFrequency {
    Unknown,
    Always,
    MoreThanOncePerWeek,
    OncePerWeek,
    LessThanOncePerWeek,
    Never,
    Any,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EntryType {
    Open,
    InviteOnly,
    Closed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BattleModifier {
    None,
    HardMode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WarResult {
    Lose,
    Win,
    Tie,
}
