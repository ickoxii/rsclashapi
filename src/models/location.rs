use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub localized_name: String,
    pub id: u32,
    pub name: String,
    pub is_country: bool,
    pub country_code: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationList(pub Vec<Location>);
