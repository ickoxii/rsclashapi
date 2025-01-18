#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use lazy_static::lazy_static;
    use reqwest;

    use rsclashapi::models::clan::*;
    use rsclashapi::models::clan_capital::*;
    use rsclashapi::models::player::*;
    use rsclashapi::models::cwl::*;
    use rsclashapi::models::war::*;
    use rsclashapi::models::war_log::*;

    use rsclashapi::utils::format_tag;

    #[derive(Debug)]
    struct Tags {
        pub th1: String,
        pub clan_no_cwl: String,
        pub clan_no_capital: String,
        pub clan_no_war: String,
        pub clan_brand_new: String,
    }

    fn get_tags() -> anyhow::Result<Tags> {
        dotenv().ok();

        Ok(Tags {
            th1: format_tag(&env::var("TH1").expect("Failed to parse TH1 from .env")),
            clan_no_cwl: format_tag(&env::var("CLAN_NO_CWL").expect("Failed to parse CLAN_NO_CWL from .env")),
            clan_no_capital: format_tag(&env::var("CLAN_NO_CAPITAL").expect("Failed to parse CLAN_NO_CAPITAL from .env")),
            clan_no_war: format_tag(&env::var("CLAN_NO_WAR").expect("Failed to parse CLAN_NO_WAR from .env")),
            clan_brand_new: format_tag(&env::var("CLAN_BRAND_NEW").expect("Failed to parse CLAN_BRAND_NEW from .env")),
        })
    }

    lazy_static! {
        static ref BASE_URL: &'static str = "https://api.clashofclans.com/v1";
        static ref API_TOKEN: String = env::var("AUTH_TOKEN_HOME").unwrap();
        pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
    }

    // Test player endpoint with th1 account
    // OK
    #[tokio::test]
    async fn test_player_endpoint() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/players/{}", *BASE_URL, tags.th1);

        println!("tag: {}", tags.th1);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let player: Player = response.json().await?;

        // assert_eq!(player.tag, tag.to_uppercase());
        assert_eq!(player.trophies, 0);

        Ok(())
    }

    // Test capital endpoints with th3 with no clan capital
    // OK
    #[tokio::test]
    async fn test_clan_no_capital() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/capitalraidseasons?limit=5", *BASE_URL, tags.clan_no_capital);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan: ClanCapitalRaidSeasons = response.json().await?;

        Ok(())
    }

    // Test clan endpoints with clan that has not been to war
    // ERROR
    #[tokio::test]
    async fn test_clan_no_war_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_war);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan: Clan = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_war_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/warlog", *BASE_URL, tags.clan_no_war);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _warlog: ClanWarLog = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_war_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/currentwar", *BASE_URL, tags.clan_no_war);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan_war: ClanWar = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_war_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_war);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _group: ClanWarLeagueGroup = response.json().await?;

        Ok(())
    }

    // Test clan endpoints with clan that has not been in cwl
    // OK
    #[tokio::test]
    async fn test_clan_no_cwl_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_cwl);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan: Clan = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_cwl_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/warlog", *BASE_URL, tags.clan_no_cwl);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _warlog: ClanWarLog = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_cwl_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/currentwar", *BASE_URL, tags.clan_no_cwl);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan_war: ClanWar = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_cwl_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_cwl);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _group: ClanWarLeagueGroup = response.json().await?;

        Ok(())
    }

    // Test clan endpoints with clan that has no clan capital
    // ERROR
    #[tokio::test]
    async fn test_clan_no_capital_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_capital);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan: Clan = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_capital_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/warlog", *BASE_URL, tags.clan_no_capital);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _warlog: ClanWarLog = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_capital_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/currentwar", *BASE_URL, tags.clan_no_capital);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan_war: ClanWar = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_no_capital_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_no_capital);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _group: ClanWarLeagueGroup = response.json().await?;

        Ok(())
    }

    // Test on brand new clan
    #[tokio::test]
    async fn test_clan_brand_new_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_brand_new);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan: Clan = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_brand_new_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/warlog", *BASE_URL, tags.clan_brand_new);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _warlog: ClanWarLog = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_brand_new_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}/currentwar", *BASE_URL, tags.clan_brand_new);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _clan_war: ClanWar = response.json().await?;

        Ok(())
    }

    // ERROR
    #[tokio::test]
    async fn test_clan_brand_new_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();

        let url = format!("{}/clans/{}", *BASE_URL, tags.clan_brand_new);

        let response = CLIENT
            .get(url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let _group: ClanWarLeagueGroup = response.json().await?;

        Ok(())
    }
}
