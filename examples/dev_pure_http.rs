// cargo run --example credentials
use anyhow;
use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest;
use serde_json::{from_str, to_string_pretty, Value};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use tokio;
use urlencoding::encode;

use rsclashapi::auth::credentials::*;
use rsclashapi::auth::dev::*;

/*
use rsclashapi::models::badge_urls::*;
use rsclashapi::models::clan::*;
use rsclashapi::models::clan_capital::*;
use rsclashapi::models::enums::*;
use rsclashapi::models::error::*;
use rsclashapi::models::gold_pass::*;
use rsclashapi::models::icon_urls::*;
use rsclashapi::models::labels::*;
use rsclashapi::models::language::*;
use rsclashapi::models::league::*;
use rsclashapi::models::location::*;
use rsclashapi::models::player::*;
use rsclashapi::models::ranking::*;
*/

fn get_credentials() -> Credentials {
    let email = env::var("EMAIL").expect("EMAIL environment variable missing");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable missing");

    let credentials = Credentials::builder()
        .add_credential(email, password)
        .build();

    credentials
}

#[derive(Debug)]
pub struct Tags {
    pub war_tag: String,
    pub player_tag: String,
    pub small_player_tag: String,
    pub lar_tag: String,
    pub lar_mini_tag: String,
    pub new_clan_tag: String,
    pub season_id: String,
    pub location_id: String,
    pub league_id: String,
    pub capital_league_id: String,
    pub builder_league_id: String,
    pub war_league_id: String,
}

fn get_tags() -> Tags {
    Tags {
        war_tag: format_tag(env::var("WAR_TAG").unwrap()),
        player_tag: format_tag(env::var("PT_SOUL").unwrap()),
        small_player_tag: format_tag(env::var("PT_TH2").unwrap()),
        lar_tag: format_tag(env::var("CT_LAR").unwrap()),
        lar_mini_tag: format_tag(env::var("CT_LAR_MINI").unwrap()),
        new_clan_tag: format_tag(env::var("CT_NEWCLAN").unwrap()),
        season_id: env::var("SEASON_ID").unwrap(),
        location_id: env::var("LOCATION_ID").unwrap(),
        league_id: env::var("LEAGUE_ID").unwrap(),
        capital_league_id: env::var("CAPITAL_LEAGUE_ID").unwrap(),
        builder_league_id: env::var("BUILDER_LEAGUE_ID").unwrap(),
        war_league_id: env::var("WAR_LEAGUE_ID").unwrap(),
    }
}

fn format_tag(tag: String) -> String {
    if tag.starts_with('#') {
        format!("%23{}", &tag[1..]) // Replace "#" with "%23"
    } else if !tag.starts_with("%23") {
        format!("%23{}", tag) // If it doesn't start with "%23", prepend it
    } else {
        tag // If it's already in "%23" format, leave it
    }
}

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

async fn fetch_and_save_to_separate_files(
    dir_name: &str,
    api_token: &str,
    endpoints: &[String],
) -> Result<(), anyhow::Error> {
    for endpoint in endpoints {
        println!("Hitting endpoint: {}", endpoint);

        let response = CLIENT
            .get(endpoint)
            .header("Authorization", format!("Bearer {}", api_token))
            .send()
            .await?;

        let raw_text = response.text().await?;

        match serde_json::from_str::<Value>(&raw_text) {
            Ok(json) => {
                let pretty_json = to_string_pretty(&json).unwrap();

                // Generate a file name based on the endpoint
                let file_name = format_file_name(dir_name, endpoint);

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(file, "{}", pretty_json)?;
            }
            Err(e) => {
                eprintln!("Failed to parse JSON from {}: {}", endpoint, e);
                eprintln!("Raw response: {}", raw_text);

                // Save error info in a separate file
                let file_name = format_file_name(dir_name, endpoint);
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(
                    file,
                    "Endpoint: {}\nFailed to parse JSON: {}\n",
                    endpoint, e
                )?;
            }
        }
    }
    Ok(())
}

async fn fetch_and_save_to_separate_files_no_auth(
    dir_name: &str,
    api_token: &str,
    endpoints: &[String],
) -> Result<(), anyhow::Error> {
    for endpoint in endpoints {
        println!("Hitting endpoint: {}", endpoint);

        let response = CLIENT.get(endpoint).send().await?;

        let raw_text = response.text().await?;

        match serde_json::from_str::<Value>(&raw_text) {
            Ok(json) => {
                let pretty_json = serde_json::to_string_pretty(&json).unwrap();

                // Generate a file name based on the endpoint
                let file_name = format_file_name(dir_name, endpoint);

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(file, "Endpoint: {}\n{}", endpoint, pretty_json)?;
            }
            Err(e) => {
                eprintln!("Failed to parse JSON from {}: {}", endpoint, e);
                eprintln!("Raw response: {}", raw_text);

                // Save error info in a separate file
                let file_name = format_file_name(dir_name, endpoint);
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(
                    file,
                    "Endpoint: {}\nFailed to parse JSON: {}\n",
                    endpoint, e
                )?;
            }
        }
    }
    Ok(())
}

async fn fetch_and_save_to_separate_files_invalid_auth(
    dir_name: &str,
    api_token: &str,
    endpoints: &[String],
) -> Result<(), anyhow::Error> {
    for endpoint in endpoints {
        println!("Hitting endpoint: {}", endpoint);

        let response = CLIENT
            .get(endpoint)
            .header("Authorization", format!("Bearer {}", 1234))
            .send()
            .await?;

        let raw_text = response.text().await?;

        match serde_json::from_str::<Value>(&raw_text) {
            Ok(json) => {
                let pretty_json = serde_json::to_string_pretty(&json).unwrap();

                // Generate a file name based on the endpoint
                let file_name = format_file_name(dir_name, endpoint);

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(file, "Endpoint: {}\n{}", endpoint, pretty_json)?;
            }
            Err(e) => {
                eprintln!("Failed to parse JSON from {}: {}", endpoint, e);
                eprintln!("Raw response: {}", raw_text);

                // Save error info in a separate file
                let file_name = format_file_name(dir_name, endpoint);
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_name)?;

                writeln!(
                    file,
                    "Endpoint: {}\nFailed to parse JSON: {}\n",
                    endpoint, e
                )?;
            }
        }
    }
    Ok(())
}

fn format_file_name(dir_name: &str, endpoint: &str) -> String {
    // Extract the endpoint from the URL (remove the base URL)
    let endpoint_parts: Vec<&str> = endpoint.split("/v1").collect();
    let endpoint_name = endpoint_parts.last().unwrap_or(&"").to_string();

    // Replace non-alphanumeric characters with underscores
    let sanitized = endpoint_name.replace(|c: char| !c.is_alphanumeric(), "_");
    format!("{}/{}.json", dir_name, sanitized)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    // Grab vars from dotenv
    let credentials = get_credentials();
    let tags: Tags = get_tags();
    println!("{:#?}", tags);

    const API_BASE_URL: &'static str = "https://api.clashofclans.com/v1";

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

    // Hit API endpoints with valid parameters and bodies
    let api_endpoints = vec![
        // ----- Clans -----
        format!(
            "{}/clans/{}/currentwar/leaguegroup",
            API_BASE_URL, tags.lar_mini_tag
        ),
        format!("{}/clanwarleagues/wars/{}", API_BASE_URL, tags.war_tag),
        format!("{}/clans/{}/warlog", API_BASE_URL, tags.lar_tag),
        format!(
            "{}/clans?name={}&limit=5",
            API_BASE_URL,
            encode("loot & run").to_string()
        ),
        format!("{}/clans/{}/currentwar", API_BASE_URL, tags.lar_tag),
        format!("{}/clans/{}", API_BASE_URL, tags.lar_tag),
        format!("{}/clans/{}/members", API_BASE_URL, tags.lar_tag),
        format!(
            "{}/clans/{}/capitalraidseasons?limit={}",
            API_BASE_URL, tags.lar_tag, 5
        ),
        // ----- Players -----
        format!("{}/players/{}", API_BASE_URL, tags.player_tag),
        // ----- Leagues -----
        format!("{}/capitalleagues", API_BASE_URL),
        format!("{}/leagues", API_BASE_URL),
        format!(
            "{}/leagues/{}/seasons/{}?limit={}",
            API_BASE_URL, tags.league_id, tags.season_id, 100
        ),
        format!("{}/capitalleagues/{}", API_BASE_URL, tags.capital_league_id),
        format!(
            "{}/builderbaseleagues/{}",
            API_BASE_URL, tags.builder_league_id
        ),
        format!("{}/builderbaseleagues", API_BASE_URL),
        format!("{}/leagues/{}", API_BASE_URL, tags.league_id),
        format!("{}/leagues/{}/seasons", API_BASE_URL, tags.league_id),
        format!("{}/warleagues/{}", API_BASE_URL, tags.war_league_id),
        format!("{}/warleagues", API_BASE_URL),
        // ----- Locations -----
        format!(
            "{}/locations/{}/rankings/clans?limit={}",
            API_BASE_URL, tags.location_id, 5
        ),
        format!(
            "{}/locations/{}/rankings/players?limit={}",
            API_BASE_URL, tags.location_id, 5
        ),
        format!(
            "{}/locations/{}/rankings/players-builder-base?limit={}",
            API_BASE_URL, tags.location_id, 5
        ),
        format!(
            "{}/locations/{}/rankings/clans-builder-base?limit={}",
            API_BASE_URL, tags.location_id, 5
        ),
        format!("{}/locations", API_BASE_URL),
        format!(
            "{}/locations/{}/rankings/capitals?limit={}",
            API_BASE_URL, tags.location_id, 5
        ),
        format!("{}/locations/{}", API_BASE_URL, tags.location_id),
        // ----- GoldPass -----
        format!("{}/goldpass/seasons/current", API_BASE_URL),
        // ----- Labels -----
        format!("{}/labels/players", API_BASE_URL),
        format!("{}/labels/clans", API_BASE_URL),
    ];

    fetch_and_save_to_separate_files(
        "raw_responses",
        &login_response.temporary_api_token,
        &api_endpoints,
    )
    .await?;

    fetch_and_save_to_separate_files_no_auth(
        "error_responses/no_token",
        &login_response.temporary_api_token,
        &api_endpoints,
    )
    .await?;
    fetch_and_save_to_separate_files_invalid_auth(
        "error_responses/invalid_token",
        &login_response.temporary_api_token,
        &api_endpoints,
    )
    .await?;

    // Invalid endpoints
    let invalid_api_endpoints = vec![
        // ----- Clans -----
        format!(
            "{}/clans/{}{}/currentwar/leaguegroup",
            API_BASE_URL, tags.lar_mini_tag, 1
        ),
        format!("{}/clanwarleagues/wars/{}{}", API_BASE_URL, tags.war_tag, 1),
        format!("{}/clans/{}{}/warlog", API_BASE_URL, tags.lar_tag, 1),
        format!(
            "{}/clans?name={}&limit=5",
            API_BASE_URL,
            encode("jfeliies").to_string()
        ),
        format!("{}/clans/{}{}/currentwar", API_BASE_URL, tags.lar_tag, 1),
        format!("{}/clans/{}{}", API_BASE_URL, tags.lar_tag, 1),
        format!("{}/clans/{}{}/members", API_BASE_URL, tags.lar_tag, 1),
        format!(
            "{}/clans/{}{}/capitalraidseasons?limit={}",
            API_BASE_URL, tags.lar_tag, 1, 5
        ),
        // ----- Players -----
        format!("{}/players/{}{}", API_BASE_URL, tags.player_tag, 1),
        // ----- Leagues -----
        format!("{}/capitalleagues", API_BASE_URL),
        format!("{}/leagues", API_BASE_URL),
        format!(
            "{}/leagues/{}{}/seasons/{}?limit={}",
            API_BASE_URL, tags.league_id, 1, tags.season_id, 100
        ),
        format!(
            "{}/leagues/{}/seasons/{}{}?limit={}",
            API_BASE_URL, tags.league_id, tags.season_id, 1, 100
        ),
        format!("{}/capitalleagues/{}{}", API_BASE_URL, tags.capital_league_id, 1),
        format!(
            "{}/builderbaseleagues/{}{}",
            API_BASE_URL, tags.builder_league_id, 1
        ),
        format!("{}/builderbaseleagues", API_BASE_URL),
        format!("{}/leagues/{}{}", API_BASE_URL, tags.league_id, 1),
        format!("{}/leagues/{}{}/seasons", API_BASE_URL, tags.league_id, 1),
        format!("{}/warleagues/{}{}", API_BASE_URL, tags.war_league_id, 1),
        format!("{}/warleagues", API_BASE_URL),
        // ----- Locations -----
        format!(
            "{}/locations/{}{}/rankings/clans",
            API_BASE_URL, tags.location_id, 1
        ),
        format!(
            "{}/locations/{}{}/rankings/players?limit={}",
            API_BASE_URL, tags.location_id, 1, 5
        ),
        format!(
            "{}/locations/{}{}/rankings/players-builder-base?limit={}",
            API_BASE_URL, tags.location_id, 1, 5
        ),
        format!(
            "{}/locations/{}{}/rankings/clans-builder-base?limit={}",
            API_BASE_URL, tags.location_id, 1, 5
        ),
        format!("{}/locations", API_BASE_URL),
        format!(
            "{}/locations/{}{}/rankings/capitals?limit={}",
            API_BASE_URL, tags.location_id, 1, 5
        ),
        format!("{}/locations/{}{}", API_BASE_URL, tags.location_id, 1),
        // ----- GoldPass -----
        format!("{}/goldpass/seasons/current", API_BASE_URL),
        // ----- Labels -----
        format!("{}/labels/players", API_BASE_URL),
        format!("{}/labels/clans", API_BASE_URL),
    ];

    fetch_and_save_to_separate_files(
        "error_responses/not_found",
        &login_response.temporary_api_token,
        &invalid_api_endpoints,
    )
    .await?;

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
