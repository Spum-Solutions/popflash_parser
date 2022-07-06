use super::team::Team;
use select::{document::Document, predicate::Attr};

use select::predicate::Class;
use select::predicate::Name;

use serde_derive::{Deserialize, Serialize};

// Matches have unique match IDs and therefore can implement Eq
#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    id: String,

    // TODO:
    // date
    // time
    // type
    // map
    // link to demo
    teams: Vec<Team>,
}

impl Eq for Match {}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Match {
    pub fn new(document: &Document) -> Self {
        Self {
            id: Self::get_match_id(document),
            teams: Match::get_teams(document),
        }
    }

    /// Retrieve match information from a match id such as `1281644`
    pub async fn from_id(match_id: usize) -> Result<Match, Box<dyn std::error::Error>> {
        let body = crate::utility::get_body_from_id(match_id).await.unwrap();
        Ok(Match::new(&body))
    }

    /// Same as `match_from_id` instead taking a full url as a `&str`
    pub async fn from_url(url: &str) -> Result<Match, Box<dyn std::error::Error>> {
        let body = crate::utility::get_body_from_url(url).await.unwrap();
        Ok(Match::new(&body))
    }

    fn get_teams(document: &Document) -> Vec<Team> {
        let mut teams: Vec<Team> = vec![];
        let team1 = Team::new(0, document);
        let team2 = Team::new(1, document);

        teams.push(team1);
        teams.push(team2);
        teams
    }

    fn get_match_id(document: &Document) -> String {
        document
            .find(Name("title"))
            .next()
            .unwrap()
            .text()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
    }
}
