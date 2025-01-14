use serde::{Deserialize, Serialize};

use super::{badge_urls, icon_urls, labels};
use crate::models::enums::player::*;
use crate::models::enums::clan::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub tag: String,
    pub name: String,
    pub town_hall_level: u8,
    pub town_hall_weapon_level: Option<u8>,
    pub exp_level: u32,
    pub trophies: u32,
    pub best_trophies: u32,
    pub war_stars: u32,
    pub attack_wins: u32,
    pub defense_wins: u32,
    pub builder_hall_level: Option<u8>,
    pub builder_base_trophies: u32,
    pub best_builder_base_trophies: u32,
    pub role: Option<Role>,
    pub war_preference: Option<WarPreference>,
    pub donations: u32,
    pub donations_received: u32,
    pub clan_capital_contributions: u32,
    pub clan: Option<PlayerClan>,
    pub league: Option<PlayerLeague>,
    pub builder_base_league: Option<BuilderBaseLeague>,
    pub legend_statistics: Option<PlayerLegendStatistics>,
    pub achievements: Vec<Achievement>,
    pub player_house: Option<PlayerHouse>,
    pub labels: Option<Vec<labels::PlayerLabels>>,
    pub troops: Vec<Troop>,
    pub heroes: Option<Vec<Hero>>,
    pub hero_equipment: Option<Vec<HeroEquipment>>,
    pub spells: Option<Vec<Spell>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub name: String,
    pub clan_level: u8,
    pub badge_urls: badge_urls::BadgeUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLeague {
    pub id: u32,
    pub name: String,
    pub icon_urls: icon_urls::LeagueIconUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuilderBaseLeague {
    pub id: u32,
    pub name: String,
}

impl WarPreference {
    #[must_use]
    pub fn is_opted_in(&self) -> bool {
        self == &Self::In
    }

    #[must_use]
    pub fn is_opted_out(&self) -> bool {
        self == &Self::Out
    }
}

impl ToString for WarPreference {
    fn to_string(&self) -> String {
        match self {
            Self::Out => "out".to_string(),
            Self::In => "in".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLegendStatistics {
    pub legend_trophies: u32,
    pub previous_season: LegendLeagueTournamentSeasonResults,
    pub best_season: LegendLeagueTournamentSeasonResults,
    pub current_season: LegendLeagueTournamentSeasonResults,
    // pub previous_builder_base_season: LegendLeagueTournamentSeasonResults,
    // pub best_builder_base_season: LegendLeagueTournamentSeasonResults,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegendLeagueTournamentSeasonResults {
    pub trophies: u32,
    // id is a "YYYY-MM" string
    pub id: Option<String>,
    pub rank: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerItemLevel {
    pub level: u32,
    pub name: String,
    pub max_level: u32,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub equipment: Option<Vec<HeroEquipment>>,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroEquipment {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Troop {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
// PlayerAchievementProgress
pub struct Achievement {
    pub name: String,
    pub stars: u8,
    pub value: u32,
    pub target: u32,
    pub info: String,
    pub completion_info: Option<String>,
    pub village: VillageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerHouse {
    pub elements: Vec<PlayerHouseElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerHouseElement {
    // Type of player house element
    pub r#type: PlayerHouseElementType,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    pub tag: String,
    pub token: String,
    pub status: String,
}

impl Player {
    const ALL_UNITS: [&'static str; 74] = [
        "Barbarian",
        "Archer",
        "Goblin",
        "Giant",
        "Wall Breaker",
        "Balloon",
        "Wizard",
        "Healer",
        "Dragon",
        "P.E.K.K.A",
        "Minion",
        "Hog Rider",
        "Valkyrie",
        "Golem",
        "Witch",
        "Lava Hound",
        "Bowler",
        "Baby Dragon",
        "Miner",
        "Super Barbarian",
        "Super Archer",
        "Super Wall Breaker",
        "Super Giant",
        "Raged Barbarian",
        "Sneaky Archer",
        "Beta Minion",
        "Boxer Giant",
        "Bomber",
        "Power P.E.K.K.A",
        "Cannon Cart",
        "Drop Ship",
        "Baby Dragon", // builder base
        "Night Witch",
        "Wall Wrecker",
        "Battle Blimp",
        "Yeti",
        "Sneaky Goblin",
        "Super Miner",
        "Rocket Balloon",
        "Ice Golem",
        "Electro Dragon",
        "Stone Slammer",
        "Inferno Dragon",
        "Super Valkyrie",
        "Dragon Rider",
        "Super Witch",
        "Hog Glider",
        "Siege Barracks",
        "Ice Hound",
        "Super Bowler",
        "Super Dragon",
        "Headhunter",
        "Super Wizard",
        "Super Minion",
        "Log Launcher",
        "Flame Flinger",
        "Battle Drill",
        "Electro Titan",
        "Apprentice Warden",
        "Super Hog Rider",
        "Electrofire Wizard",
        "Root Rider",
        "Druid",
        "Thrower",
        "L.A.S.S.I",
        "Mighty Yak",
        "Electro Owl",
        "Unicorn",
        "Phoenix",
        "Poison Lizard",
        "Diggy",
        "Frosty",
        "Spirit Fox",
        "Angry Jelly",
    ];

    /*
    const HOME_TROOP_ORDER: [&'static str; 10] = [
    ];

    const SUPER_TROOP_ORDER: [&'static str; 10] = [
    ];

    const BUILDER_TROOP_ORDER: [&'static str; 10] = [
    ];

    const SIEGE_MACHINE_ORDER: [&'static str; 10] = [
    ];
    */

    const HERO_ORDER: [&'static str; 7] = [
        "Barbarian King",
        "Archer Queen",
        "Grand Warden",
        "Battle Machine",
        "Royal Champion",
        "Battle Copter",
        "Minion Prince",
    ];

    const HERO_EQ_ORDER: [&'static str; 26] = [
        "Giant Gauntlet",
        "Rocket Spear",
        "Spiky Ball",
        "Frozen Arrow",
        "Fireball",
        "Magic Mirror",
        "Electro Boots",
        "Lavaloon Puppet",
        "Barbarian Puppet",
        "Rage Vial",
        "Archer Puppet",
        "Invisibility Vial",
        "Eternal Tome",
        "Life Gem",
        "Seeking Shield",
        "Royal Gem",
        "Earthquake Boots",
        "Hog Rider Puppet",
        "Vampstache",
        "Haste Vial",
        "Giant Arrow",
        "Healer Puppet",
        "Rage Gem",
        "Healing Tome",
        "Henchmen Puppet",
        "Dark Orb",
    ];

    const SPELL_ORDER: [&'static str; 15] = [
        "Lightning Spell",
        "Healing Spell",
        "Rage Spell",
        "Jump Spell",
        "Freeze Spell",
        "Poison Spell",
        "Earthquake Spell",
        "Haste Spell",
        "Clone Spell",
        "Skeleton Spell",
        "Bat Spell",
        "Invisibility Spell",
        "Recall Spell",
        "Overgrowth Spell",
        "Revive Spell",
    ];

    /*
    const PET_ORDER: &'static str; 10] = [
    ];

    const CLAN_CAPITAL_TROOP_ORDER: &'static str; 10] = [
    ];

    const CLAN_CAPITAL_SPELL_ORDER: &'static str; 10] = [
    ];
    */

    const ACHIEVEMENT_ORDER: [&'static str; 47] = [
        "Bigger Coffers",
        "Get those Goblins!",
        "Bigger & Better",
        "Nice and Tidy",
        "Discover New Troops",
        "Gold Grab",
        "Elixir Escapade",
        "Sweet Victory!",
        "Empire Builder",
        "Wall Buster",
        "Humiliator",
        "Union Buster",
        "Conqueror",
        "Unbreakable",
        "Friend in Need",
        "Mortar Mauler",
        "Heroic Heist",
        "League All-Star",
        "X-Bow Exterminator",
        "Firefighter",
        "War Hero",
        "Clan War Wealth",
        "Anti-Artillery",
        "Sharing is caring",
        "Keep Your Account Safe!", // connect to social network
        "Master Engineering",
        "Next Generation Model",
        "Un-Build It",
        "Champion Builder",
        "High Gear",
        "Hidden Treasures",
        "Games Champion",
        "Dragon Slayer",
        "War League Legend",
        "Keep Your Account Safe!", // connect to supercell id
        "Well Seasoned",
        "Shattered and Scattered",
        "Not So Easy This Time",
        "Bust This!",
        "Superb Work",
        "Siege Sharer",
        "Aggressive Capitalism",
        "Most Valuable Clanmate",
        "Counterspell",
        "Monolith Masher",
        "Ungrateful Child",
        "Supercharger",
    ];
}
