/// Errors returned from the Clash of Clans API
use serde::{Serialize, Deserialize};

/// I don't know when this error is returned but it is listed in the supercell
/// API documentation page.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientAPIError {
    pub reason: Option<String>,
    pub message: Option<String>,
    // Type of api error
    pub r#type: Option<String>,
    pub detail: Option<String>, // Unsure what type this actually is
}

/// This error response is sent when no session is found
/// -- i.e. 401 Error, Unauthorized
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientDevError {
    pub error: Option<String>,
    pub description: Option<String>,
}

/// This error response is sent when invalid authorization is found when
/// communicating with the API.
/// -- i.e. 403 Error, Forbidden
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupercellAPIError {
    pub reason: Option<String>,
    pub message: Option<String>,
}

