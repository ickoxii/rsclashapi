use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    pub name: String,
    pub id: u16,
    #[serde(rename = "languageCode")]
    pub language_code: String,
}
