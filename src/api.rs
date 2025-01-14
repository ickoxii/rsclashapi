// This file should contain stuff regarding the actual api
pub mod api {
    const BASE_URL: &'static str = "https://api.clashofclans.com/v1";

    // ----- CLANS -----
    // Retrieve information about clan's current war league group
    // Return: ClanWarLeagueGroup
    fn league_group_endpoint(clantag: &str) -> String {
        format!("/clans/{}/currentwar/leaguegroup", clantag)
    }

    // Retrieve information about individual clan war league war
    // Return: ClanWarLeagueGroup
    fn warleague_war_endpoint(war_tag: &str) -> String {
        format!("/clanwarleagues/wars/{}", war_tag)
    }

    // Retrieve clan's clan war log
    // Parameters:
    //  clanTag(String): tag of the clan,
    //  limit(integer): limit number of items returned in response,
    //  after(String): return only items that occur after this marker,
    //  before(String): return only items that occur before this marker,
    // Return: ClanWarLog
    fn warlog_endpoint(clantag: &str) -> String {
        format!("/clans/{}/warlog", clantag)
    }

    // Search clans
    // Parameters:
    //  name(String > 3 chars): search clans by name. name parameter is
    //      interpreted as a wildcard search,
    //  warFrequency(String): filter by war frequency,
    //  locationId(int): filter clans by location identifier,
    //  minMembers(int): filter clans by minimum number of members,
    //  maxMembers(int): filter clans by maximum number of members,
    //  minClanPoints(int): filter by minimum amount of clan points,
    //  minClanLevel(int): filter by clan level,
    //  limit(int): limit number of items returned in response,
    //  after(String): return only items that occur after this marker,
    //  before(String): return only items that occur before this marker,
    //  labelIds(String): comma separated list of label ids used for filtering results,
    // Return: ClanList
    fn search_clans_endpoint() -> &'static str {
        "/clans"
    }

    // Retrieve information about clan's current clan war
    // Return: ClanWar
    fn curr_war_endpoint(clantag: &str) -> String {
        format!("/clans/{}/currentwar", clantag)
    }

    // Clan Information
    // Return: Clan
    fn clan_endpoint(clantag: &str) -> String {
        format!("/clans/{}", clantag)
    }

    // List clan members
    // Parameters:
    //  clanTag(String): tag of clan,
    //  limit(int): limit number of items returned in response,
    //  after(String): return only items that occur after this marker,
    //  before(String): return only items that occur before this marker,
    // Return: ClanMemberList
    fn clan_members_endpoint(clantag: &str) -> String {
        format!("/clans/{}/members", clantag)
    }

    // Retrieve clan's capital raid seasons
    // Parameters:
    //  clanTag(String): tag of clan,
    //  limit(int): limit number of items returned in response,
    //  after(String): return only items that occur after this marker,
    //  before(String): return only items that occur before this marker,
    // Return: ClanCapitalRaidSeasons
    fn clan_capital_raid_seasons_endpoint(clantag: &str) -> String {
        format!("/clans/{}/capitalraidseasons", clantag)
    }


    // ----- PLAYERS -----
    // Get player information
    // Return: Player
    fn player_endpoint(player_tag: &str) -> String {
        format!("/players/{}", player_tag)
    }

    // POST: Verify player API token that can be found from the game settings
    // Return: VerifyTokenResponse
    fn verify_token_endpoint(player_tag: &str) -> String {
        format!("/players/{}/verifytoken", player_tag)
    }


    // ----- LEAGUES -----
    // List capital leagues
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Returns: LeagueList
    fn capital_leagues_endpoint() -> &'static str {
        "/capitalleagues"
    }

    // List leagues
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Returns: LeagueList
    fn leagues_endpoint() -> &'static str {
        "/leagues"
    }

    // Get league season rankings
    // Parameters:
    //  leagueId(String): identifier of the league,
    //  seasonId(String): identifier of the season,
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: PlayerRankingList
    fn league_season_rankings_endpoint(league_id: &str, season_id: &str) -> String {
        format!("/leagues/{}/seasons/{}", league_id, season_id)
    }

    // Get capital league information
    // Return: League
    fn capital_leagues_info_endpoint(league_id: &str) -> String {
        format!("/capitalleagues/{}", league_id)
    }

    // Get builder base league information
    // Return: League
    fn builder_base_leagues_info_endpoint(league_id: &str) -> String {
        format!("/builderbaseleagues/{}", league_id)
    }

    // List builder base leagues
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: LeagueList
    fn builder_base_leagues_endpoint() -> &'static str {
        "/builderbaseleagues"
    }

    // Get league information
    // Return: League
    fn league_info_endpoint(league_id: &str) -> String {
        format!("/leagues/{}", league_id)
    }

    // Get league seasons
    // Parameters:
    //  leagueId(String): identifier of the league
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: LeagueSeasonList
    fn league_seasons_endpoint(league_id: &str) -> String {
        format!("/leagues/{}/seasons", league_id)
    }

    // Get war league information
    // Return: League
    fn war_league_info_endpoint(league_id: &str) -> String {
        format!("/warleagues/{}", league_id)
    }

    // List war leagues
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: LeagueList
    fn war_leagues_endpoint() -> &'static str {
        "/warleagues"
    }


    // ----- LOCATIONS -----
    // Get clan rankings for a specific location
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: ClanRankingList
    fn local_clan_rankings_endpoint(location_id: &str) -> String {
        format!("/locations/{}/rankings/clans", location_id)
    }

    // Get player rankings for a specific location
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: PlayerRankingList
    fn local_player_rankings_endpoint(location_id: &str) -> String {
        format!("/locations/{}/rankings/players", location_id)
    }

    // Get player Builder Base rankings for a specific location
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: PlayerBuilderBaseRankingList
    fn local_player_builder_rankings_endpoint(location_id: &str) -> String {
        format!("/locations/{}/rankings/players-builder-base", location_id)
    }

    // Get clan Builder Base rankings for a specific location
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: ClanBuilderBaseRankingList
    fn local_clan_builder_rankings_endpoint(location_id: &str) -> String {
        format!("/locations/{}/rankings/clans-builder-base", location_id)
    }

    // List locations
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: LocationList
    fn locations_endpoint() -> &'static str {
        "/locations"
    }

    // Get capital rankings for a specific location
    // Parameters:
    //  limit(integer): limit number of items in response,
    //  after(String): return only items that occur after marker,
    //  before(String): return only items that occur before marker,
    // Return: ClanCapitalRankingList
    fn local_capital_rankings_endpoint(location_id: &str) -> String {
        format!("/locations/{}/rankings/capitals", location_id)
    }

    // Get location information
    // Return: Location
    fn locations_info_endpoint(location_id: &str) -> String {
        format!("/locations/{}", location_id)
    }


    // ----- GOLDPASS -----
    // Get information about the current gold pass season
    // Returns: GoldPassSeason
    fn goldpass_endpoint() -> &'static str {
        "/goldpass/seasons/current"
    }


    // ----- LABELS -----
    // List player labels
    fn player_labels_endpoint() -> &'static str {
        "/labels/players"
    }

    // List clan labels
    fn clan_labels_endpoint() -> &'static str {
        "/labels/clans"
    }
}

