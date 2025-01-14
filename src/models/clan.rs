use serde::{Deserialize, Serialize};
use super::badge_urls::BadgeUrls;
use super::league::League;
use super::player::PlayerHouse;
use super::language::Language;
use super::labels::ClanLabels;
use super::location::Location;
use super::clan_capital::ClanCapital;
use super::enums::clan::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub war_league: Option<League>,
    pub capital_league: Option<League>,
    pub member_list: Option<ClanMemberList>,
    pub tag: String,
    pub chat_language: Option<Language>,
    pub is_war_log_public: bool,
    pub clan_capital_points: Option<u32>,
    pub required_trophies: Option<u16>,
    pub required_builder_base_trophies: Option<u16>,
    pub required_townhall_level: Option<u16>,
    pub clan_builder_base_points: Option<u32>,
    pub is_family_friendly: bool,
    pub war_losses: u32,
    pub clan_points: u32,
    pub clan_level: u16,
    pub war_win_streak: u16,
    pub war_wins: u32,
    pub war_ties: u32,
    pub war_frequency: Option<WarFrequency>,
    pub labels: Option<Vec<ClanLabels>>,
    pub name: String,
    pub location: Location,
    pub r#type: EntryType,
    pub members: u8,
    pub description: String,
    pub clan_capital: ClanCapital,
    pub badge_urls: BadgeUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanList(pub Vec<Clan>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub league: League,
    pub builder_base_league: League,
    pub tag: String,
    pub name: String,
    pub role: Role,
    pub town_hall_level: u8,
    pub exp_level: u16,
    pub clan_rank: u8,
    pub previous_clan_rank: u8,
    pub donations: u32,
    pub donations_received: u32,
    pub trophies: u16,
    pub builder_base_trophies: u16,
    pub player_house: PlayerHouse,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanMemberList(pub Vec<ClanMember>);

// ----- Clan War Leagues -----
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLeagueGroup {
    pub tag: String,
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

// ----- Clan War Log -----
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarLog(pub Vec<ClanWarLogEntry>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLogEntry {
    pub clan: WarClan,
    pub opponent: WarClan,
    pub team_size: u8,
    pub attacks_per_member: u8,
    pub battle_modifier: BattleModifier,
    pub end_time: String,
    pub result: WarResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WarClan {
    pub destruction_percentage: f32,
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
    pub clan_level: u8,
    pub attacks: u8,
    pub stars: u16,
    pub exp_earned: u16,
    pub members: ClanWarMemberList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    pub map_position: u8,
    pub townhall_level: u8,
    pub opponent_attacks: u8,
    pub best_opponent_attack: ClanWarAttack,
    pub attacks: ClanWarAttackList,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarMemberList(pub Vec<ClanWarMember>);

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClanWarAttackList(pub Vec<ClanWarAttack>);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWar {
    pub clan: WarClan,
    pub opponent: WarClan,
    pub team_size: u8,
    pub attacks_per_member: u8,
    pub battle_modifier: BattleModifier,
    pub start_time: String,
    pub state: WarState,
    pub end_time: String,
    pub preparation_start_time: String,
}
