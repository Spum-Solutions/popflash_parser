pub const EXAMPLE_GAME_URL: &'static str = "https://popflash.site/match/1281644";
pub const EXAMPLE_GAME_ID: &'static str = "1281644";

pub mod player_stats;
pub mod team;
pub mod utility;

// has to have capital to not conflict with rust keywords
pub mod Match;

pub async fn match_from_id(match_id: &str) -> Match::Match {
    let body = utility::get_body_from_id(match_id).await.unwrap();
    Match::Match::new(match_id, &body)
}

pub async fn match_from_url(url: &str) -> Match::Match {
    let body = utility::get_body_from_url(url).await.unwrap();
    let match_id = url
        .trim_start_matches("https://popflash.site/match/")
        .trim_end_matches('/');

    Match::Match::new(match_id, &body)
}
