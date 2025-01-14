// cargo run --example credentials
use dotenv::dotenv;
use reqwest;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::env;
use tokio;
use anyhow;
use lazy_static::lazy_static;

use rsclashapi::auth::credentials::*;
use rsclashapi::auth::dev::*;
use rsclashapi::models::player::Player;
use rsclashapi::auth::keys::Keys;

fn get_credentials() -> Credentials {
    let email = env::var("EMAIL").expect("EMAIL environment variable missing");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable missing");

    let credentials = Credentials::builder()
        .add_credential(email, password)
        .build();

    credentials
}

fn format_tag(tag: &str) -> String {
    if tag.starts_with('#') {
        format!("%23{}", &tag[1..])
    } else {
        format!("%23{}", tag)
    }
}

lazy_static! {
    pub static ref CLIENT: reqwest::Client =
        reqwest::Client::builder().cookie_store(true).build().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    // Get player tag
    let soul_tag = format_tag(&env::var("PT_SOUL").expect("PT_SOUL environment variable missing"));
    // println!("PT_SOUL: {soul_tag}");

    // Grab credentials from dotenv
    let credentials = get_credentials();

    // -----Login-----
    // Important -- make sure to build cookie store
    // let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let body = serde_json::to_string(&credentials.0.first().unwrap())?;
    let res = CLIENT
        .post("https://developer.clashofclans.com/api/login")
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // -----Save information into an APIAccount Object-----
    let login_response: LoginResponse = from_str(&res.text().await?)?;

    // pretty print the debug information
    // println!("LoginResponse\n{:#?}", login_response);

    // -----Get Keys from list-----
    let key_list = CLIENT
        .post("https://developer.clashofclans.com/api/apikey/list")
        .send()
        .await?;

    let keys: Keys = from_str(&key_list.text().await?)?;

    // let credential = credentials.0.first().unwrap().clone();

    let api_account = APIAccount {
        credentials,
        response: login_response.clone(),
        keys,
    };

    // println!("\nAPIAccount\n{:#?}", api_account);

    // println!("\nTempToken\n{}", &login_response.temporary_api_token);

    // -----Get Player Info-----
    let player_response = CLIENT
        .get(format!("https://api.clashofclans.com/v1/players/{}", soul_tag))
        .header("Authorization", format!("Bearer {}", &login_response.temporary_api_token))
        .send()
        .await?;

    let raw_response = player_response.text().await?;

    let player: Player = serde_json::from_str::<Player>(&raw_response)?;
    println!("\nPlayer\n{:#?}", player);

    // -----Logout-----
    let res = CLIENT
        .post("https://developer.clashofclans.com/api/logout")
        .send()
        .await?;

    let text = res.text().await?;
    match serde_json::from_str::<Value>(&text) {
        Ok(json) => {
            // Pretty-print the JSON
            println!("\nLogout\n{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }

    Ok(())
}
