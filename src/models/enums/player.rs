use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WarPreference {
    Out,
    In,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VillageType {
    #[serde(rename = "home")]
    HomeVillage,
    #[serde(rename = "builderBase")]
    BuilderBase,
    #[serde(rename = "clanCapital")]
    ClanCapital,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PlayerHouseElementType {
    Ground,
    Roof,
    Walls,
    Decoration,
}
