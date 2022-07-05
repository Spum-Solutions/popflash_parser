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

use r#match::Match;

mod player_stats;
mod team;
pub mod utility;

// match is a rust keyword so need to be done like this
pub mod r#match;

/// Retrieve match information from a match id such as `1281644`
pub async fn match_from_id(match_id: usize) -> Match {
    let body = utility::get_body_from_id(match_id).await.unwrap();
    Match::new(&body)
}

/// Same as `match_from_id` instead taking a full url as a `&str`
pub async fn match_from_url(url: &str) -> Match {
    let body = utility::get_body_from_url(url).await.unwrap();
    Match::new(&body)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_GAME_URL: &str = "https://popflash.site/match/1281644";
    const EXAMPLE_GAME_ID: usize = 1281644;
    mod match_from_id {
        use super::*;
        #[tokio::test]
        async fn valid_popflash_id_1() {
            let body = utility::get_body_from_id(EXAMPLE_GAME_ID).await.unwrap();
            todo!()
        }

        #[test]
        fn invalid_popflash_id_1() {
            todo!()
        }
    }

    mod match_from_url {
        #[test]
        fn valid_popflash_url_1() {
            todo!()
        }

        #[test]
        fn invalid_popflash_url_1() {
            todo!()
        }
    }
}
