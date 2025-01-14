use serde::{Serialize, Deserialize};
use super::super::models::status::Status;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyResponse {
    pub status: Option<Status>,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: Option<i32>,
    pub keys: Option<Keys>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keys {
    pub keys: Vec<Key>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Key {
    pub id: String,
    #[serde(rename = "developerId")]
    pub developer_id: String,
    pub tier: String,
    pub name: String,
    pub description: String,
    pub origins: Option<String>,
    pub scopes: Vec<String>,
    #[serde(rename = "cidrRanges")]
    pub cidr_ranges: Vec<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<String>,
    pub key: String,
}

impl Keys {
    #[must_use]
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

