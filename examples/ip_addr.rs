// cargo run --example ip_addr
use reqwest;
use std::net::IpAddr;
use std::str::FromStr;

async fn get_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    let public_ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    let ip_addr = IpAddr::from_str(&public_ip)?; // Parse the IP into an IpAddr
    Ok(ip_addr)
}

#[tokio::main]
async fn main() {
    match get_ip().await {
        Ok(ip) => println!("Public IP Address: {}", ip),
        Err(e) => eprintln!("Error: {}", e),
    }
}
