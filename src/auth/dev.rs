use lazy_static::lazy_static;
/// This file contains functionality regarding login/out
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::sync::Mutex;
use anyhow;

use super::super::error::APIError;
use super::super::models::status::Status;
use super::super::utils::get_ip;
use super::credentials::Credentials;
use super::keys::{Key, Keys};

#[derive(Debug)]
pub struct APIAccount {
    pub credentials: Credentials,
    pub response: LoginResponse,
    pub keys: Keys,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub status: Status,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: u32,
    pub auth: Option<Auth>,
    pub developer: Developer,
    #[serde(rename = "temporaryAPIToken")]
    pub temporary_api_token: TemporaryApiToken,
    #[serde(rename = "swaggerUrl")]
    pub swagger_url: String,
}

// Logging into your supercell account will return a temporaryAPIToken as
// a string. The format and body of the token can be found by pasting this
// token into <https://jwt.io/>
// Header:
// {"typ": "JWT","alg": "HS512","kid": "28a318f7-0000-a1eb-7fa1-2c7433c6cca5"}
// Payload:
// {"iss": "supercell","aud": "supercell:gameapi","jti": "fdfa9ef0-f9af-1ecf-4554-473b54297655",
// "iat": 1736552500,"exp": 1736556100,"sub": "developer/431654b0-b27a-eb07-d536-6b58bb47529e",
// "scopes": ["clash"],"limits": [{"tier": "developer/bronze","type": "throttling"},
// {"cidrs": ["73.92.85.0/32"],"type": "client"},{"origins": ["developer.clashofclans.com"],
// "type": "cors"}]}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporaryApiToken {
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub iat: i64,
    pub exp: i64,
    pub sub: String,
    pub scopes: Vec<Scope>,
    pub limits: Vec<Limit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Limit {
    pub tier: Option<String>,
    pub cidrs: Option<Vec<String>>,
    pub origins: Option<Vec<String>>,
    pub r#type: String,
}

// This is passed when logging out or revoking a key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogoutResponse {
    pub status: Status,
    #[serde(rename = "sessionExpiresInSeconds")]
    pub session_expires_in_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    pub uid: String,
    pub token: String,
    pub ua: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Developer {
    pub id: String,
    pub name: String,
    pub game: String,
    pub email: String,
    pub tier: String,
    pub allowed_scopes: Option<String>,
    pub max_cidrs: Option<String>,
    pub prev_login_ts: Option<String>,
    pub prev_login_ip: Option<String>,
    pub prev_login_ua: Option<String>,
}

lazy_static! {
    pub static ref DEV_API_URL: Mutex<String> =
        Mutex::new(String::from("https://developer.clashofclans.com/api"));
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

impl APIAccount {
    pub const LOGIN_ENDPOINT: &'static str = "/login";
    pub const LOGOUT_ENDPOINT: &'static str = "/logout";
    pub const KEY_CREATE_ENDPOINT: &'static str = "/apikey/create";
    pub const KEY_REVOKE_ENDPOINT: &'static str = "/apikey/revoke";
    pub const KEY_LIST_ENDPOINT: &'static str = "/apikey/list";

    /// Update the keys of the APIAccount after creating or revoking a key.
    pub fn update_keys(&mut self, keys: Keys) {
        self.keys = keys;
    }

    /// Login to the supercell api using an email and password
    pub async fn login(email: &str, password: &str) -> anyhow::Result<Self, APIError> {
        // Init here because i have no idea what the fuck im doing
        let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

        // Create credentials
        let credentials = Credentials::builder()
            .add_credential(email.to_string(), password.to_string())
            .build();

        // Serialize the credentials to JSON
        let credential_body = serde_json::to_string(
            credentials
                .0
                .first()
                .ok_or_else(|| APIError::InvalidCredentials)?,
        )
        .map_err(|e| APIError::SerializationFailed(e.to_string()))?; // Ensure error is converted to APIError

        let base_url = DEV_API_URL.lock().unwrap();

        // Send login request
        let res = client
            .post(format!("{}{}", base_url, Self::LOGIN_ENDPOINT))
            .body(credential_body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(APIError::RequestFailed)?;

        // Check login response
        if res.status().is_success() {
            let keys = Self::list_keys().await?;
            Ok(Self {
                credentials: credentials.clone(),
                response: res
                    .json::<LoginResponse>()
                    .await
                    .map_err(APIError::RequestFailed)?,
                keys,
            })
        } else {
            Err(APIError::from_response(res).await)
        }
    }

    /// Lists all keys tied to a supercell API account
    pub async fn list_keys() -> anyhow::Result<Keys, APIError> {
        let base_url = DEV_API_URL.lock().unwrap();

        let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

        let key_list_res = client
            .post(format!("{}{}", base_url, Self::KEY_LIST_ENDPOINT))
            .send()
            .await
            .map_err(APIError::RequestFailed)?;

        if key_list_res.status().is_success() {
            let status = key_list_res.status();
            let body = key_list_res.text().await.map_err(APIError::RequestFailed)?;
            let keys: Keys =
                from_str(&body).map_err(|e| APIError::BadResponse(e.to_string(), status))?;
            Ok(keys)
        } else {
            Err(APIError::from_response(key_list_res).await)
        }
    }

    pub async fn create_key(
        key_name: &str,
        account: &mut APIAccount
    ) -> anyhow::Result<Key, APIError> {
        // Retrieve the public IP address
        let ip_address = get_ip()
            .await
            .map_err(|e| APIError::FailedGetIp(e.to_string()))?;

        // Build the request body as a JSON object
        let key_body = serde_json::json!({
            "name": key_name,
            "cidrRanges": [ip_address],
            "description": "Key generated via rsclashapi",
            "scopes": null
        });

        // Serialize the body into a string
        let body = serde_json::to_string(&key_body)
            .map_err(|e| APIError::SerializationFailed(e.to_string()))?;

        let base_url = DEV_API_URL.lock().unwrap();

        // Send the request to create the key
        let res = CLIENT
            .post(format!("{}{}", base_url, Self::KEY_CREATE_ENDPOINT))
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(APIError::RequestFailed)?;

        // Handle the response
        if res.status().is_success() {
            let status = res.status();
            let body = res.text().await.map_err(APIError::RequestFailed)?;
            let key: Key = serde_json::from_str(&body)
                .map_err(|e| APIError::BadResponse(e.to_string(), status))?;

            // Update the APIAccount's keys with the newly created key
            let mut updated_keys = account.keys.clone(); // Clone existing keys
            updated_keys.keys.push(key.clone()); // Add the new key to the list
            account.update_keys(updated_keys); // Update the account's keys

            Ok(key)
        } else {
            Err(APIError::from_response(res).await) // Handle any failed responses
        }
    }

    pub async fn revoke_key(
        key_id: &str,
        account: &mut APIAccount,
    ) -> anyhow::Result<LogoutResponse, APIError> {
        let base_url = DEV_API_URL.lock().unwrap();

        // Build the URL for the revocation request
        let url = format!("{}{}", base_url, Self::KEY_REVOKE_ENDPOINT);

        // Create the request body with the key ID
        let request_body = serde_json::json!({
            "id": key_id,
        });

        // Serialize the body to JSON
        let body = serde_json::to_string(&request_body)
            .map_err(|e| APIError::SerializationFailed(e.to_string()))?;

        // Send the request to revoke the key
        let res = CLIENT
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(APIError::RequestFailed)?;

        // Handle the response
        if res.status().is_success() {
            let status = res.status();
            let body = res.text().await.map_err(APIError::RequestFailed)?;
            let response: LogoutResponse = serde_json::from_str(&body)
                .map_err(|e| APIError::BadResponse(e.to_string(), status))?;

            // Remove the revoked key from the APIAccount's keys
            let updated_keys: Vec<Key> = account
                .keys
                .keys
                .iter()
                .filter(|key| key.id != key_id) // Filter out the revoked key
                .cloned() // Clone the remaining keys
                .collect(); // Collect the remaining keys into a Vec<Key>

            // Update the account's keys
            account.keys.keys = updated_keys;

            Ok(response)
        } else {
            Err(APIError::from_response(res).await) // Handle any failed responses
        }
    }
}
