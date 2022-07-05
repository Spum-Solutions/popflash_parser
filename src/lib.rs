#![deny(missing_docs)]

//! `popflash_parser` is a crate to parse and translate match information from <https://popflash.site/> This tool is currently working as of February 27th 2022, although other similar tools have been borked when `popflash` has changed it's web page structure so take caution when using.
//!
//! Methods are used to get the match data from a url or match ID, they are functionally equivalent to the end user, only differing in what is used to call the function (a url, or match ID)
//!
//! ``` rust
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

#[doc(hidden)]
const EXAMPLE_MATCH_URL: &str = "https://popflash.site/match/1281644";

#[doc(hidden)]
const EXAMPLE_MATCH_ID: usize = 1281644;

use r#match::Match;

mod player_stats;
mod team;

#[doc(hidden)]
pub mod utility;

// match is a rust keyword so need to be done like this
#[doc(hidden)]
pub mod r#match;

/// Retrieve match information from a match id such as `1281644`
pub async fn match_from_id(match_id: usize) -> Result<Match, Box<dyn std::error::Error>> {
    let body = utility::get_body_from_id(match_id).await.unwrap();
    Ok(Match::new(&body))
}

/// Same as `match_from_id` instead taking a full url as a `&str`
pub async fn match_from_url(url: &str) -> Result<Match, Box<dyn std::error::Error>> {
    let body = utility::get_body_from_url(url).await.unwrap();
    Ok(Match::new(&body))
}

#[cfg(test)]
mod tests {
    use super::*;
    mod match_from_id {
        use super::*;
        #[tokio::test]
        // valid test as this is to check that popflash hasn't changed it's output
        async fn valid_popflash_id_1() {
            assert!(match_from_id(EXAMPLE_MATCH_ID).await.is_ok());
        }

        #[test]
        fn invalid_popflash_id_1() {
            todo!()
        }
    }

    mod match_from_url {
        use super::*;
        #[tokio::test]
        async fn valid_popflash_url_1() {
            assert!(match_from_url(EXAMPLE_MATCH_URL).await.is_ok())
        }

        #[test]
        fn invalid_popflash_url_1() {
            todo!()
        }
    }
}
