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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WarLeagueState {
    GroupNotFound,
    NotInWar,
    Preparation,
    War,
    Ended,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryType {
    Open,
    InviteOnly,
    Closed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BattleModifier {
    None,
    HardMode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WarResult {
    Lose,
    Win,
    Tie,
}
