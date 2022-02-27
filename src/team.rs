use select::document::Document;
use select::predicate::Class;
use serde_derive::{Deserialize, Serialize};

use super::player_stats::PlayerStats;

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    // represent team 1 or team 2 - makes it easier to distinguish which group of players stats to get etc
    pub id: u8,
    pub rounds_won: u8,
    pub players: Vec<PlayerStats>,
}

impl Team {
    pub fn new(team_id: u8, document: &Document) -> Self {
        Team {
            id: team_id,
            rounds_won: Self::get_rounds_won(team_id, document),
            players: self::Team::get_players_stats(team_id, document),
        }
    }

    // constructor related functions
    // functions return the value to make testing easier (not necessarily needed thoughh)
    fn get_rounds_won(team_id: u8, document: &Document) -> u8 {
        document
            .find(Class(format!("score-{}", team_id + 1).as_ref()))
            .next()
            .unwrap()
            .text()
            .trim()
            .parse::<u8>()
            .unwrap()
    }

    fn get_players_stats(team_id: u8, document: &Document) -> Vec<PlayerStats> {
        let mut team: Vec<PlayerStats> = vec![];

        for player_number in 0..5 {
            //gets the name
            // all the stats follow which is still needed
            let mut player_name = document
                .find(Class("name"))
                .nth(player_number + (20 * team_id) as usize)
                .unwrap()
                .text();
            if !player_name.chars().next().unwrap().is_ascii() {
                player_name.remove(0);
            }
            player_name = player_name.trim().to_string();
            team.push(super::player_stats::PlayerStats::new(player_name, document));
        }
        team
    }
}
