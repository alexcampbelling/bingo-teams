use crate::data_structures::Player;
use reqwest::Error;
use serde_json::Value;

const DAY_IN_SEC: i32 = 86400;
const AVG_DAYS: i32 = 365;

pub async fn create_players_with_temple(player_names: &Vec<String>) -> Result<Vec<Player>, Error> {
    let mut players = Vec::new();

    for name in player_names.iter() {
        // Get most current data
        let res_j: Value = reqwest::get(format!(
            "https://templeosrs.com/api/player_stats.php?player={}&bosses=1",
            name
        ))
        .await?
        .json()
        .await?;

        // Get some more recent gains data
        let res_avg_j: Value = reqwest::get(format!(
            "https://templeosrs.com/api/player_gains.php?player={}&time={}&bosses=1",
            name,
            DAY_IN_SEC * AVG_DAYS
        ))
        .await?
        .json()
        .await?;

        // Create a player structure with all data relevant to Temple OSRS
        // TODO ALEX: serde::json might allow a way to make this a one liner, with all the data?
        // https://levelup.gitconnected.com/working-with-csv-data-in-rust-7258163252f8
        // apparently might be slow to deserialize?
        players.push(Player {
            name: name.clone(),
            slayer: res_j["data"]["Slayer"].as_u64().unwrap(),
            ehp: res_j["data"]["Ehp"].as_f64().unwrap(),
            ehb: res_j["data"]["Ehb"].as_f64().unwrap(),
            ehp_avg: res_avg_j["data"]["Ehp"].as_f64().unwrap(),
            ehb_avg: res_avg_j["data"]["Ehb"].as_f64().unwrap(),
            tiles_score: tile_score(res_j),
            ..Default::default()
        });
    }
    Ok(players)
}

// TODO ALEX: complete this
// Tile score indicates a 'compatibility score' of the board, proportional to board access
fn tile_score(p: Value) -> f64 {
    let result: f64 = 1.;
    result
}
