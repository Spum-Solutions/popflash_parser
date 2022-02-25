use super::team::Team;
use select::document::Document;

use serde_derive::{Deserialize, Serialize};

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

impl Match {
    pub fn new(match_id: &str, document: &Document) -> Self {
        Self {
            id: match_id.to_string(),
            teams: Match::get_teams(document),
        }
    }

    fn get_teams(document: &Document) -> Vec<Team> {
        let mut teams: Vec<Team> = vec![];
        let team1 = Team::new(0, &document);
        let team2 = Team::new(1, &document);

        teams.push(team1);
        teams.push(team2);
        teams
    }
}
