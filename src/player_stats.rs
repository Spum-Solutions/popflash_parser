use select::document::Document;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats {
    name: String,
    // note kills can be negative
    kills: i8,
    assists: u8,
    deaths: u8,
    flash_assists: u8,
    // I'm not actually sure whether this can be negative if team kill?
    // if not make it u32 or similar
    average_damage_per_round: i32,
    hltv_score: f32,
    headshot_percentage: f32,
    clutch_kills: u8,
    bombs_planted: u8,
    bombs_defused: u8,
    is_supporter: bool,
}

impl PlayerStats {
    pub fn new(player_name: String, document: &Document) -> PlayerStats {
        let mut _is_supporter = false;
        let mut trimmed_player_name = player_name;
        if trimmed_player_name.starts_with('🌵') {
            //player is supporter
            trimmed_player_name = trimmed_player_name
                .chars()
                .into_iter()
                .filter(|character| character.is_ascii())
                .collect();
            _is_supporter = true;
        }

        let lines = document.nth(0).unwrap().html();
        let new_lines: Vec<String> = lines
            .lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|element| element.to_string())
            .collect();

        let mut to_take: Vec<String> = vec![];
        for (line_number, line) in new_lines.iter().enumerate() {
            if line
                .contains(format!("<td class=\"name\" title=\"{}\">", trimmed_player_name).as_str())
            {
                for i in 0..13 {
                    to_take.push(new_lines[line_number + i].clone());
                }
                break;
            }
        }

        let mut final_lines: Vec<String> = vec![];
        for line in to_take {
            if line.contains("<td>") {
                final_lines.push(line);
            }
        }

        // TODO: has to be a better way to do this
        // perhaps macro?
        
        PlayerStats {
            name: trimmed_player_name,
            kills: self::PlayerStats::parse_td_tags(&final_lines[0])
                .parse()
                .unwrap(),
            assists: self::PlayerStats::parse_td_tags(&final_lines[1])
                .parse()
                .unwrap(),
            deaths: self::PlayerStats::parse_td_tags(&final_lines[2])
                .parse()
                .unwrap(),
            flash_assists: self::PlayerStats::parse_td_tags(&final_lines[3])
                .parse()
                .unwrap(),
            average_damage_per_round: self::PlayerStats::parse_td_tags(&final_lines[4])
                .parse()
                .unwrap(),
            hltv_score: self::PlayerStats::parse_td_tags(&final_lines[5])
                .parse()
                .unwrap(),
            headshot_percentage: self::PlayerStats::parse_td_tags(&final_lines[6])
                .parse()
                .unwrap(),
            clutch_kills: self::PlayerStats::parse_td_tags(&final_lines[7])
                .parse()
                .unwrap(),
            bombs_planted: self::PlayerStats::parse_td_tags(&final_lines[8])
                .parse()
                .unwrap(),
            bombs_defused: self::PlayerStats::parse_td_tags(&final_lines[9])
                .parse()
                .unwrap(),
            is_supporter: _is_supporter,
        }
    }

    fn parse_td_tags(text: &str) -> String {
        text.trim_start_matches("<td>")
            .trim_end_matches("</td>")
            .to_string()
    }
}
