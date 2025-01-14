/// Formats a player or clan tag to begin with the url encoding for a hashtag
/// i.e. '#' -> '%23'
pub fn format_tag(tag: &str) -> String {
    if tag.starts_with('#') {
        format!("%23{}", &tag[1..])
    } else {
        format!("%23{}", tag)
    }
}

/// Get a users public ip address
pub async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org")
        .await
        .map_err(|e| format!("Failed to fetch ip: {}", e))?;
    let ip = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))?;
    Ok(ip)
}

#[cfg(test)]
pub mod test_utils {
    use dotenv::dotenv;
    use std::env;
    use crate::auth::credentials::Credentials;

    pub fn get_credentials() -> Credentials {
        dotenv().ok();

        let email = env::var("EMAIL").expect("EMAIL environment variable missing");
        let password = env::var("PASSWORD").expect("PASSWORD environment variable missing");

        let credentials = Credentials::builder()
            .add_credential(email, password)
            .build();

        credentials
    }
}
