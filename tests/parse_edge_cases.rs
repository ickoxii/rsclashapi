#[cfg(test)]
mod tests {
    use anyhow;
    use dotenv::dotenv;
    use lazy_static::lazy_static;
    use reqwest;
    use std::env;

    use rsclashapi::error::{APIError, ErrorKind};
    use rsclashapi::models::clan::*;
    use rsclashapi::models::clan_capital::*;
    use rsclashapi::models::cwl::*;
    use rsclashapi::models::player::*;
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
        pub clan_brand_new_pub: String,
    }

    fn get_tags() -> anyhow::Result<Tags> {
        dotenv().ok();

        Ok(Tags {
            th1: format_tag(&env::var("TH1").expect("Failed to parse TH1 from .env")),
            clan_no_cwl: format_tag(
                &env::var("CLAN_NO_CWL").expect("Failed to parse CLAN_NO_CWL from .env"),
            ),
            clan_no_capital: format_tag(
                &env::var("CLAN_NO_CAPITAL").expect("Failed to parse CLAN_NO_CAPITAL from .env"),
            ),
            clan_no_war: format_tag(
                &env::var("CLAN_NO_WAR").expect("Failed to parse CLAN_NO_WAR from .env"),
            ),
            clan_brand_new: format_tag(
                &env::var("CLAN_BRAND_NEW").expect("Failed to parse CLAN_BRAND_NEW from .env"),
            ),
            clan_brand_new_pub: format_tag(
                &env::var("CLAN_BRAND_NEW_PUB")
                    .expect("Failed to parse CLAN_BRAND_NEW_PUB from .env"),
            ),
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

    // Player helper function
    async fn fetch_player() -> Result<Player, APIError> {
        let tags = get_tags().unwrap();

        let tag = tags.th1;
        println!("Testing player tag: {}", tag);

        let url = format!("{}/players/{}", *BASE_URL, tag);

        let response = CLIENT
            .get(&url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await
            .map_err(APIError::from)?;

        if !response.status().is_success() {
            return Err(APIError::from_response(response).await);
        }

        let player: Player = response.json().await.map_err(|err| {
            eprintln!("Failed to deserialize player response: {:?}", err);
            APIError::SerializationFailed("Deserialization failed".into())
        })?;

        Ok(player)
    }

    // Test player endpoint with th1 account
    // TODO: look into this endpoint hit. Occasionally returns this
    // Error: 
    //  RequestFailed(reqwest::Error 
    //      {
    //          kind: Request,
    //          url: Url {
    //              scheme: "https",
    //              cannot_be_a_base: false,
    //              username: "",
    //              password: None,
    //              host: Some(Domain("api.clashofclans.com")),
    //              port: None,
    //              path: "/v1/players/%23QU8CG0JVG",
    //              query: None,
    //              fragment: None
    //          },
    //          source: hyper::Error(User(DispatchGone),
    //          "runtime dropped the dispatch task")
    //      })

    #[tokio::test]
    async fn test_player_endpoint() -> Result<(), APIError> {
        let player = fetch_player().await?;

        // Perform assertions
        assert_eq!(
            format_tag(&player.tag[1..].to_uppercase()),
            get_tags().unwrap().th1.to_uppercase(),
            "Player tag mismatch"
        );
        assert_eq!(player.trophies, 0, "Unexpected trophy count");

        // Optionally, return or print the player for debugging
        println!("{:?}", player);

        Ok(())
    }

    async fn fetch_clan_data<T>(endpoint: &str, tag: &str) -> Result<T, APIError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/clans/{}/{}", *BASE_URL, tag, endpoint);

        let response = CLIENT
            .get(&url)
            .header("Authorization", format!("Bearer {}", *API_TOKEN))
            .send()
            .await
            .map_err(APIError::from)?;

        if !response.status().is_success() {
            return Err(APIError::from_response(response).await);
        }

        response.json().await.map_err(|err| {
            eprintln!("Failed to deserialize response from {}: {:?}", url, err);
            APIError::SerializationFailed(format!("Deserialization failed for {}", endpoint))
        })
    }

    async fn fetch_clan(tag: &str) -> Result<Clan, APIError> {
        fetch_clan_data("", tag).await
    }

    async fn fetch_clan_warlog(tag: &str) -> Result<ClanWarLog, APIError> {
        fetch_clan_data("warlog", tag).await
    }

    async fn fetch_clan_currentwar(tag: &str) -> Result<ClanWar, APIError> {
        fetch_clan_data("currentwar", tag).await
    }

    async fn fetch_clan_currentwarleague(tag: &str) -> Result<ClanWarLeagueGroup, APIError> {
        fetch_clan_data("currentwar/leaguegroup", tag).await
    }

    async fn fetch_clan_capitalraidseasons(tag: &str) -> Result<ClanCapitalRaidSeasons, APIError> {
        fetch_clan_data("capitalraidseasons?limit=5", tag).await
    }

    #[tokio::test]
    async fn test_player_no_capital() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_capital;

        match fetch_clan_capitalraidseasons(&tag).await {
            Ok(capital_seasons) => println!("{:?}", capital_seasons),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_war_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_war;

        match fetch_clan(&tag).await {
            Ok(clan) => println!("{:?}", clan),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_war_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_war;

        match fetch_clan_warlog(&tag).await {
            Ok(warlog) => println!("{:?}", warlog),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_war_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_war;

        match fetch_clan_currentwar(&tag).await {
            Ok(currentwar) => println!("{:?}", currentwar),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_war_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_war;

        match fetch_clan_currentwarleague(&tag).await {
            Ok(leaguegroup) => println!("{:?}", leaguegroup),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_cwl_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_cwl;

        match fetch_clan(&tag).await {
            Ok(clan) => println!("{:?}", clan),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_cwl_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_cwl;

        match fetch_clan_warlog(&tag).await {
            Ok(warlog) => println!("{:?}", warlog),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_cwl_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_cwl;

        match fetch_clan_currentwar(&tag).await {
            Ok(currentwar) => println!("{:?}", currentwar),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_cwl_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_cwl;

        match fetch_clan_currentwarleague(&tag).await {
            Ok(leaguegroup) => println!("{:?}", leaguegroup),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_capital_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_capital;

        match fetch_clan(&tag).await {
            Ok(clan) => println!("{:?}", clan),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_capital_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_capital;

        match fetch_clan_warlog(&tag).await {
            Ok(warlog) => println!("{:?}", warlog),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_capital_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_capital;

        match fetch_clan_currentwar(&tag).await {
            Ok(currentwar) => println!("{:?}", currentwar),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_no_capital_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_no_capital;

        match fetch_clan_currentwarleague(&tag).await {
            Ok(leaguegroup) => println!("{:?}", leaguegroup),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new;

        match fetch_clan(&tag).await {
            Ok(clan) => println!("{:?}", clan),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new;

        match fetch_clan_warlog(&tag).await {
            Ok(warlog) => println!("{:?}", warlog),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new;

        match fetch_clan_currentwar(&tag).await {
            Ok(currentwar) => println!("{:?}", currentwar),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new;

        match fetch_clan_currentwarleague(&tag).await {
            Ok(leaguegroup) => println!("{:?}", leaguegroup),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_pub_clan() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new_pub;

        match fetch_clan(&tag).await {
            Ok(clan) => println!("{:?}", clan),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_pub_warlog() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new_pub;

        match fetch_clan_warlog(&tag).await {
            Ok(warlog) => println!("{:?}", warlog),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_pub_currentwar() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new_pub;

        match fetch_clan_currentwar(&tag).await {
            Ok(currentwar) => println!("{:?}", currentwar),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clan_brand_new_pub_currentwarleague() -> anyhow::Result<()> {
        let tags = get_tags().unwrap();
        let tag = tags.clan_brand_new_pub;

        match fetch_clan_currentwarleague(&tag).await {
            Ok(leaguegroup) => println!("{:?}", leaguegroup),
            Err(e) => match e.kind() {
                ErrorKind::SerializationError => {
                    println!("Serialization/Deserialization Error: {}", e)
                }
                _ => println!("HTTP or other Error: {}", e),
            },
        }

        Ok(())
    }
}
