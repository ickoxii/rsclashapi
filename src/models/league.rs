use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::icon_urls::LeagueIconUrls;
use super::paging::Paging;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u32)]
pub enum CapitalLeagueId {
    Unranked = 85_000_000,
    BronzeIII = 85_000_001,
    BronzeII = 85_000_002,
    BronzeI = 85_000_003,
    SilverIII = 85_000_004,
    SilverII = 85_000_005,
    SilverI = 85_000_006,
    GoldIII = 85_000_007,
    GoldII = 85_000_008,
    GoldI = 85_000_009,
    CrystalIII = 85_000_010,
    CrystalII = 85_000_011,
    CrystalI = 85_000_012,
    MasterIII = 85_000_013,
    MasterII = 85_000_014,
    MasterI = 85_000_015,
    ChampIII = 85_000_016,
    ChampII = 85_000_017,
    ChampI = 85_000_018,
    TitanIII = 85_000_019,
    TitanII = 85_000_020,
    TitanI = 85_000_021,
    Legend = 85_000_022,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u32)]
pub enum LeagueId {
    Unranked = 29_000_000,
    BronzeIII = 29_000_001,
    BronzeII = 29_000_002,
    BronzeI = 29_000_003,
    SilverIII = 29_000_004,
    SilverII = 29_000_005,
    SilverI = 29_000_006,
    GoldIII = 29_000_007,
    GoldII = 29_000_008,
    GoldI = 29_000_009,
    CrystalIII = 29_000_010,
    CrystalII = 29_000_011,
    CrystalI = 29_000_012,
    MasterIII = 29_000_013,
    MasterII = 29_000_014,
    MasterI = 29_000_015,
    ChampIII = 29_000_016,
    ChampII = 29_000_017,
    ChampI = 29_000_018,
    TitanIII = 29_000_019,
    TitanII = 29_000_020,
    TitanI = 29_000_021,
    Legend = 29_000_022,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u32)]
pub enum WarLeagueId {
    Unranked = 48_000_000,
    BronzeIII = 48_000_001,
    BronzeII = 48_000_002,
    BronzeI = 48_000_003,
    SilverIII = 48_000_004,
    SilverII = 48_000_005,
    SilverI = 48_000_006,
    GoldIII = 48_000_007,
    GoldII = 48_000_008,
    GoldI = 48_000_009,
    CrystalIII = 48_000_010,
    CrystalII = 48_000_011,
    CrystalI = 48_000_012,
    MasterIII = 48_000_013,
    MasterII = 48_000_014,
    MasterI = 48_000_015,
    ChampIII = 48_000_016,
    ChampII = 48_000_017,
    ChampI = 48_000_018,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u32)]
pub enum BuilderLeagueId {
    WoodV = 44_000_000,
    WoodIV = 44_000_001,
    WoodIII = 44_000_002,
    WoodII = 44_000_003,
    WoodI = 44_000_004,
    ClayV = 44_000_005,
    ClayIV = 44_000_006,
    ClayIII = 44_000_007,
    ClayII = 44_000_008,
    ClayI = 44_000_009,
    StoneV = 44_000_010,
    StoneIV = 44_000_011,
    StoneIII = 44_000_012,
    StoneII = 44_000_013,
    StoneI = 44_000_014,
    CopperV = 44_000_015,
    CopperIV = 44_000_016,
    CopperIII = 44_000_017,
    CopperII = 44_000_018,
    CopperI = 44_000_019,
    BrassIII = 44_000_020,
    BrassII = 44_000_021,
    BrassI = 44_000_022,
    IronIII = 44_000_023,
    IronII = 44_000_024,
    IronI = 44_000_025,
    SteelIII = 44_000_026,
    SteelII = 44_000_027,
    SteelI = 44_000_028,
    TitaniumIII = 44_000_029,
    TitaniumII = 44_000_030,
    TitaniumI = 44_000_031,
    PlatIII = 44_000_032,
    PlatII = 44_000_033,
    PlatI = 44_000_034,
    EmeraldIII = 44_000_035,
    EmeraldII = 44_000_036,
    EmeraldI = 44_000_037,
    RubyIII = 44_000_038,
    RubyII = 44_000_039,
    RubyI = 44_000_040,
    Diamond = 44_000_041,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub name: String, // JsonLocalizedName
    pub id: LeagueId, // might have to change to u32
    pub icon_urls: Option<LeagueIconUrls>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueList {
    pub items: Vec<League>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapitalLeague {
    pub name: String,
    pub id: CapitalLeagueId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapitalLeagueList {
    pub items: Vec<CapitalLeague>,
    pub paging: Paging
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WarLeague {
    pub name: String,
    pub id: WarLeagueId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WarLeagueList {
    pub items: Vec<WarLeague>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuilderLeague {
    pub name: String,
    pub id: BuilderLeagueId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuilderLeagueList {
    pub items: Vec<BuilderLeague>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueSeason {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeagueSeasonList {
    pub items: Vec<LeagueSeason>,
    pub paging: Paging,
}
