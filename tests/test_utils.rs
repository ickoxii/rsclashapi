#[macro_use]
extern crate dotenv_codegen;

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use rsclashapi::utils::format_tag;

    #[test]
    fn test_format_tag() {
        assert_eq!("%23abcdef", format_tag("#abcdef"));
        assert_eq!("%23abcdef", format_tag("abcdef"));
        assert_eq!("%23abc#def", format_tag("#abc#def"));
        assert_eq!("%23abc#def", format_tag("abc#def"));
        assert_eq!("%23abc#def", format_tag("%23abc#def"));
        assert_eq!("%23abc#def", format_tag("%23abc#def"));
    }

    #[test]
    fn test_get_env() {
        dotenv().ok();
        assert_ne!("", env::var("PT_SOUL").unwrap());
        assert_ne!("", env::var("PT_SOUL_MINI").unwrap());
        assert_ne!("", env::var("PT_SOUL_DONO").unwrap());
        assert_ne!("", env::var("PT_SOUL_NEO").unwrap());
        assert_ne!("", env::var("PT_SOUL_SHIJIE").unwrap());
        assert_ne!("", env::var("PT_SOUL_XIAOYANG").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH11").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH10").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH9").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH8").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH6").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH3").unwrap());
        assert_ne!("", env::var("PT_SOUL_TH2").unwrap());
        assert_ne!("", env::var("PT_TH2").unwrap());
        assert_ne!("", env::var("CT_LAR_MINI").unwrap());
        assert_ne!("", env::var("CT_LAR_CWL").unwrap());
        assert_ne!("", env::var("CT_ICKOSYSTEM").unwrap());
        assert_ne!("", env::var("CT_YOMI").unwrap());
        assert_ne!("", env::var("CT_NEWCLAN").unwrap());
        assert_ne!("", env::var("CT_LAR").unwrap());
        assert_ne!("", env::var("WAR_TAG").unwrap());
        assert_ne!("", env::var("SEASON_ID").unwrap());
        assert_ne!("", env::var("LOCATION_ID").unwrap());
        assert_ne!("", env::var("LEAGUE_ID").unwrap());
        assert_ne!("", env::var("CAPITAL_LEAGUE_ID").unwrap());
        assert_ne!("", env::var("BUILDER_LEAGUE_ID").unwrap());
        assert_ne!("", env::var("WAR_LEAGUE_ID").unwrap());
        assert_ne!("", env::var("TH1").unwrap());
        assert_ne!("", env::var("CLAN_NO_CWL").unwrap());
        assert_ne!("", env::var("CLAN_NO_CAPITAL").unwrap());
        assert_ne!("", env::var("CLAN_NO_WAR").unwrap());
        assert_ne!("", env::var("CLAN_BRAND_NEW").unwrap());
    }
}
