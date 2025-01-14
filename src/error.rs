/// Errors regarding the rsclashapi wrapper
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    /// Client is not set up
    #[error("Client is not set up")]
    NotReady,
    /// Invalid credentials on login (email or password)
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// Failed to parse URL
    #[error("Failed to parse URL")]
    BadUrl(url::ParseError),
    /// Request to API failed
    #[error("Request failed")]
    RequestFailed(reqwest::Error),
    /// Invalid header values
    #[error("Invalid header")]
    InvalidHeader(InvalidHeaderValue),
    /// Failed to retrieve IP address
    #[error("Failed to get ip address: {0}")]
    FailedGetIp(String),
    /// Bad parameters on request -- error code 400
    #[error("Client provided incorrect parameters")]
    BadParameters,
    /// Access denied for resource -- error code 403
    #[error("Access denied")]
    AccessDenied,
    /// Resource not found -- error code 404
    #[error("Resource not found")]
    NotFound,
    /// Requests to API throttled -- error code 429
    #[error("Request throttled")]
    Throttle,
    /// Unknown client error -- error code 500
    #[error("Unknown error: {0}")]
    Unknown(String),
    /// Server under maintenance -- error code 503
    #[error("Server under maintenance")]
    Maintenance,
    /// Invalid parameters passed to request
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    /// Bad response from API request
    #[error("Bad response: {0}")]
    BadResponse(String, reqwest::StatusCode),
    /// Invalid tag (player, clan, war, league, etc.)
    #[error("Invalid tag: {0}")]
    InvalidTag(String),
    /// Failed to serialize or deserialize data
    #[error("Serialization or deserialization failed: {0}")]
    SerializationFailed(String),
}

impl APIError {
    /// Maps an HTTP status code to an appropriate `APIError`
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        match status {
            reqwest::StatusCode::BAD_REQUEST => APIError::BadParameters,
            reqwest::StatusCode::FORBIDDEN => APIError::AccessDenied,
            reqwest::StatusCode::NOT_FOUND => APIError::NotFound,
            reqwest::StatusCode::TOO_MANY_REQUESTS => APIError::Throttle,
            reqwest::StatusCode::SERVICE_UNAVAILABLE => APIError::Maintenance,
            _ => APIError::BadResponse(body, status),
        }
    }
}

impl From<reqwest::Error> for APIError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestFailed(e)
    }
}

impl From<url::ParseError> for APIError {
    fn from(e: url::ParseError) -> Self {
        Self::BadUrl(e)
    }
}

impl From<InvalidHeaderValue> for APIError {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(e)
    }
}
