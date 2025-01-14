use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelIconUrls {
    pub small: String,
    pub medium: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeagueIconUrls {
    pub small: String,
    pub tiny: String,
    pub medium: Option<String>,
}
