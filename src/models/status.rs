use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub code: i16,
    pub message: String,
    pub detail: Option<String>,
}
