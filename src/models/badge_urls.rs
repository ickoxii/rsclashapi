use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BadgeUrls {
    small: String,
    medium: String,
    large: String,
}
