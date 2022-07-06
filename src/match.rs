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

    fn popflash_time_to_utc(
        document: &Document,
    ) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        // TODO: this needs a major refactor
        // example popflash date looks like "06/02/2022 AT 20:05:43"

        let mut date = document.find(Name("h2")).into_iter().next().unwrap().text();
        let mut time = date.clone();

        date.replace_range((0..date.find("ON ").unwrap() + 3), "");
        date.replace_range((date.find(' ').unwrap()..), "");

        time.replace_range((0..time.find("AT ").unwrap() + 3), "");
        time.replace_range((time.find(' ').unwrap()..), "");

        let date_iter = date.split('/');
        let mut time_iter = time.split(':');

        let dates = date_iter.collect::<Vec<_>>();

        let datetime = Utc
            .ymd(
                dates[2].parse::<i32>().unwrap(),
                dates[0].parse::<u32>().unwrap(),
                dates[1].parse::<u32>().unwrap(),
            )
            .and_hms(
                time_iter.next().unwrap().parse::<u32>().unwrap(),
                time_iter.next().unwrap().parse::<u32>().unwrap(),
                time_iter.next().unwrap().parse::<u32>().unwrap(),
            );

        Ok(datetime)
    }

    fn get_map_name(document: &Document) -> String {
        let lines = document.nth(0).unwrap().html();
        let new_lines: Vec<String> = lines
            .lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|element| element.to_string())
            .collect();
        let map_name = new_lines
            .iter()
            .position(|line| line.contains("score score-2"))
            .unwrap()
            + 6;
        let map_name = new_lines[map_name].clone();

        map_name
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

#[cfg(test)]
mod tests {
    const EXAMPLE_GAME_URL: &str = "https://popflash.site/match/1281644";
    const EXAMPLE_GAME_ID: usize = 1281644;
    mod from_id {
        use super::*;
        #[tokio::test]
        async fn valid_popflash_id_1() {
            let body = crate::utility::get_body_from_id(EXAMPLE_GAME_ID)
                .await
                .unwrap();
            todo!()
        }

        #[test]
        fn invalid_popflash_id_1() {
            todo!()
        }
    }

    mod from_url {
        #[test]
        fn valid_popflash_url_1() {
            todo!()
        }

        #[test]
        fn invalid_popflash_url_1() {
            todo!()
        }
    }

    mod popflash_time_to_utc {
        fn test_valid_1() {
            todo!()
        }
    }
}
