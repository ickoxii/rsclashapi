use serde::{Serialize, Deserialize};

use super::paging::Paging;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub localized_name: Option<String>,
    pub id: u32,
    pub name: String,
    pub is_country: bool,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationList {
    pub items: Vec<Location>,
    pub paging: Paging,
}
