#[cfg(test)]
mod tests {
    use serde_json;
    use std::fs;

    use rsclashapi::models::clan::*;
    use rsclashapi::models::clan_capital::*;
    use rsclashapi::models::gold_pass::*;
    use rsclashapi::models::labels::*;
    use rsclashapi::models::league::*;
    use rsclashapi::models::location::*;
    use rsclashapi::models::player::*;
    use rsclashapi::models::ranking::*;
    use rsclashapi::models::cwl::*;
    use rsclashapi::models::war::*;
    use rsclashapi::models::war_log::*;

    fn format_file_name(dir_name: &str, endpoint: &str) -> String {
        // Extract the endpoint from the URL (remove the base URL)
        let endpoint_parts: Vec<&str> = endpoint.split("/v1").collect();
        let endpoint_name = endpoint_parts.last().unwrap_or(&"").to_string();

        // Replace non-alphanumeric characters with underscores
        let sanitized = endpoint_name.replace(|c: char| !c.is_alphanumeric(), "_");
        format!("{}/{}.json", dir_name, sanitized)
    }

    fn read_and_parse_json<T: for<'de> serde::Deserialize<'de>>(
        file_path: &str,
    ) -> serde_json::Result<T> {
        let file_content: String = fs::read_to_string(file_path).expect("Unable to read file");

        serde_json::from_str(&file_content)
    }

    const BASE_DIR: &'static str = "raw_responses";

    // ----- Clans -----
    #[test]
    fn parse_currentwar_leaguegroup() {
        const FILE: &'static str = "_clans__232GP8Q2RYL_currentwar_leaguegroup.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let league_group: ClanWarLeagueGroup =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", league_group);
    }

    #[test]
    fn parse_clanwarleagues_wars() {
        const FILE: &'static str = "_clanwarleagues_wars__238YGPV2L2J.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        // TODO: I think this returns a clan war object instead of a ClanWarLeagueGroup
        let league_group: ClanWar = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", league_group);
    }

    #[test]
    fn parse_clan_warlog() {
        const FILE: &'static str = "_clans__23902J8QYY_warlog.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let war_log: ClanWarLog = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", war_log);
    }

    #[test]
    fn parse_clan_search() {
        const FILE: &'static str = "_clans_name_loot_20_26_20run_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        // TODO: Change to Vec<Clan>?
        let clan_list: ClanList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", clan_list);
    }

    #[test]
    fn parse_clan_currentwar() {
        const FILE: &'static str = "_clans__23902J8QYY_currentwar.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let clan_war: ClanWar = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", clan_war);
    }

    #[test]
    fn parse_clan_info() {
        const FILE: &'static str = "_clans__23902J8QYY.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let clan: Clan = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", clan);
    }

    #[test]
    fn parse_clan_members() {
        const FILE: &'static str = "_clans__23902J8QYY_members.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let clan_members: ClanMemberList =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", clan_members);
    }

    #[test]
    fn parse_clan_capital_raid_seasons() {
        const FILE: &'static str = "_clans__23902J8QYY_capitalraidseasons_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let seasons_list: ClanCapitalRaidSeasons =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", seasons_list);
    }

    // ----- Players -----
    #[test]
    fn parse_player() {
        const FILE: &'static str = "_players__23C2J2QRRQ.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let player: Player = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", player);
    }

    // ----- Leagues -----
    #[test]
    fn parse_capital_leagues() {
        const FILE: &'static str = "_capitalleagues.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let capital_leagues: CapitalLeagueList =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", capital_leagues);
    }

    #[test]
    fn parse_leagues() {
        const FILE: &'static str = "_leagues.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let leagues: LeagueList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", leagues);
    }

    #[test]
    fn parse_league_season_info() {
        const FILE: &'static str = "_leagues_29000022_seasons_2024_12_limit_100.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let player_rankings: PlayerRankingList =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", player_rankings);
    }

    #[test]
    fn parse_capital_leagues_info() {
        const FILE: &'static str = "_capitalleagues_85000022.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let capital_league: CapitalLeague =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", capital_league);
    }

    #[test]
    fn parse_builder_league_info() {
        const FILE: &'static str = "_builderbaseleagues_44000041.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let builder_league: BuilderLeague =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", builder_league);
    }

    #[test]
    fn parse_builder_leagues() {
        const FILE: &'static str = "_builderbaseleagues.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let builder_leagues: BuilderLeagueList =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", builder_leagues);
    }

    #[test]
    fn parse_league_info() {
        const FILE: &'static str = "_leagues_29000022.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let league: League = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", league);
    }

    #[test]
    fn parse_league_seasons() {
        const FILE: &'static str = "_leagues_29000022_seasons.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let seasons_list: LeagueSeasonList =
            read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", seasons_list);
    }

    #[test]
    fn parse_warleague_info() {
        const FILE: &'static str = "_warleagues_48000018.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let war_league: WarLeague = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", war_league);
    }

    #[test]
    fn parse_warleagues() {
        const FILE: &'static str = "_warleagues.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let leagues: WarLeagueList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", leagues);
    }

    // ----- Locations -----
    #[test]
    fn parse_location_clan_rankings() {
        const FILE: &'static str = "_locations_32000249_rankings_clans_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let rankings: ClanRankingList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", rankings);
    }

    #[test]
    fn parse_location_player_rankings() {
        const FILE: &'static str = "_locations_32000249_rankings_players_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let rankings: PlayerRankingList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", rankings);
    }

    #[test]
    fn parse_location_builder_player_rankings() {
        const FILE: &'static str = "_locations_32000249_rankings_players_builder_base_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let rankings: PlayerBuilderBaseRankingList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", rankings);
    }

    #[test]
    fn parse_location_builder_clan_rankings() {
        const FILE: &'static str = "_locations_32000249_rankings_clans_builder_base_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let rankings: ClanBuilderBaseRankingList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", rankings);
    }

    #[test]
    fn parse_locations() {
        const FILE: &'static str = "_locations.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let locations: LocationList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", locations);
    }

    #[test]
    fn parse_location_capital_rankings() {
        const FILE: &'static str = "_locations_32000249_rankings_capitals_limit_5.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let rankings: ClanCapitalRankingList = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", rankings);
    }

    #[test]
    fn parse_location_info() {
        const FILE: &'static str = "_locations_32000249.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let location: Location = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", location);
    }

    // ----- GoldPass -----
    #[test]
    fn parse_goldpass() {
        const FILE: &'static str = "_goldpass_seasons_current.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let goldpass: GoldPassSeason = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", goldpass);
    }

    // ----- Labels -----
    #[test]
    fn parse_player_labels() {
        const FILE: &'static str = "_labels_players.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let labels: PlayerLabelResponse = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", labels);
    }

    #[test]
    fn parse_clan_labels() {
        const FILE: &'static str = "_labels_clans.json";
        let path = format!("{}/{}", BASE_DIR, FILE);
        let labels: ClanLabelResponse = read_and_parse_json(&path).expect("Failed to parse json");
        println!("{:#?}", labels);
    }
}
