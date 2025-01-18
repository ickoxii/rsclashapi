use serde::{Serialize, Deserialize};

use crate::models::enums::clan::WarLeagueState;
use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLeagueGroup {
    pub tag: Option<String>,
    pub state: WarLeagueState,
    pub season: String,
    pub clans: ClanWarLeagueClanList,
    pub rounds: ClanWarLeagueRoundList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueMember {
    pub tag: String,
    pub town_hall_level: u8,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLeagueMemberList(pub Vec<ClanWarLeagueMember>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueClan {
    pub tag: String,
    pub clan_level: u8,
    pub name: String,
    pub members: ClanWarLeagueMemberList,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLeagueClanList(pub Vec<ClanWarLeagueClan>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueRound {
    pub war_tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLeagueRoundList(pub Vec<ClanWarLeagueRound>);

