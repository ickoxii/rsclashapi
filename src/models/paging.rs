use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paging {
    pub cursors: Cursor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub before: Option<String>,
    pub after: Option<String>,
}
