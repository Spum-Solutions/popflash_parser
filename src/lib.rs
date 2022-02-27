//! `popflash_parser` is a crate to parse and translate match information from <https://popflash.site/> This tool is currently working as of February 27th 2022, although other similar tools have been borked when `popflash` has changed it's web page structure so take caution when using.
//!
//! Methods are used to get the match data from a url or match ID, the are functionally equivalent to the end user only differing in what is used to call the function
//!
//!
//! ```
//! # use popflash_parser::*;
//! ##[tokio::test]
//! # async fn test() {
//! assert_eq!(
//!     match_from_url("https://popflash.site/match/1281644").await,
//!     match_from_id("1281644").await
//! );
//! # }
//! ```
//!
//! A `Match` struct (see example output here: <https://pastebin.com/MKUyjx9b>) can be printed as follows
//! (run `cargo test -- --nocapture` to see the whole output);
//!
//! ```
//! use popflash_parser::*;
//! # async fn test() {
//! let match_data  = match_from_id("1281644").await;
//! println!("Match::Match output");
//! println!("{:#?}", match_data);
//!
//! // Convert to JSON string
//! let json_string = serde_json::to_string(&match_data).unwrap();
//! println!("String output");
//! println!("{}", json_string);
//!
//! // Convert  to JSON object
//! let json_object = serde_json::Value::from(json_string);
//! println!("serde_json::Value output");
//! println!("{}", json_object);
//! # }
//! ```

pub const EXAMPLE_GAME_URL: &str = "https://popflash.site/match/1281644";
pub const EXAMPLE_GAME_ID: &str = "1281644";

pub mod player_stats;
pub mod team;
pub mod utility;

// has to have capital to not conflict with rust keywords
#[warn(non_snake_case)]
pub mod Match;

/// Retrieve match information from a match id such as `1281644`
pub async fn match_from_id(match_id: &str) -> Match::Match {
    let body = utility::get_body_from_id(match_id).await.unwrap();
    Match::Match::new(&body)
}

/// Same as `match_from_id` instead taking a full url as a `&str`
pub async fn match_from_url(url: &str) -> Match::Match {
    let body = utility::get_body_from_url(url).await.unwrap();
    Match::Match::new(&body)
}

/// Offers synchronous support for the crate with the same interface of async context is unavailable.
pub mod synchronous {
    use tokio::runtime::Runtime;

    /// Retrieve match information from a match id such as `1281644`
    pub fn match_from_id(match_id: &str) -> crate::Match::Match {
        let rt = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        let match_data =
            rt.block_on(async { super::utility::get_body_from_id(match_id).await.unwrap() });
        crate::Match::Match::new(&match_data)
    }

    /// Retrieve match information from a match url such as <https://popflash.site/match/1281644>
    pub fn match_from_url(url: &str) -> crate::Match::Match {
        let rt = Runtime::new().unwrap();

        // Execute the future, blocking the current thread until completion
        let match_data =
            rt.block_on(async { super::utility::get_body_from_url(url).await.unwrap() });
        crate::Match::Match::new(&match_data)
    }
}
