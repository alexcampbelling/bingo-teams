use crate::data_structures::Player;
use reqwest::Error;
use serde_json::Value;

const DAY_IN_SEC: u64 = 86400;
const DAYS_IN_YEAR: u64 = 365;
const AVERAGE_DAYS_COEFF: u64 = 14; // Used to make a nicer score for efficient hours stats
const SLAYER_95_XP: u64 = 8771558;

// let BOSSES_IN_BOARD: Vec<String> = [String::from("Zulrah"), String::from("butt")];

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
            DAY_IN_SEC * DAYS_IN_YEAR
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
            // Here we also double check Temple gave data, panic on failure
            // TODO ALEX: Handle this more gracefully
            slayer_ability: slayer_ability(
                res_j["data"]["Slayer"].as_u64().expect(&format!(
                    "Couldn't retrieve player: {}, check their Temple page!",
                    name
                )),
                SLAYER_95_XP,
            ),
            // TODO ALEX: scale these numbers before adding weights? something relative to a normal weeks worth of hours?
            ehp: res_j["data"]["Ehp"].as_f64().unwrap(),
            ehb: res_j["data"]["Ehb"].as_f64().unwrap(),
            ehp_avg: average_efficient_hours(res_avg_j["data"]["Ehp"].as_f64().unwrap()),
            ehb_avg: average_efficient_hours(res_avg_j["data"]["Ehb"].as_f64().unwrap()),
            tiles_score: get_tile_pc(res_j.clone()),
            ..Default::default()
        });
    }
    Ok(players)
}

// Here we state if player can kill Hydra (Normally highest boss on bingo, tells us they have access to all Slayer)
// TODO ALEX: skew this data to similar to xp exponential skew to fit
fn slayer_ability(xp: u64, xp_wanted: u64) -> f64 {
    // TODO ALEX: make this cleaner, should be able to one liner
    if xp < xp_wanted {
        return xp as f64 / xp_wanted as f64 * 100.0;
    } else {
        return 100.0;
    }
}

fn average_efficient_hours(hours: f64) -> f64 {
    hours / (DAYS_IN_YEAR / AVERAGE_DAYS_COEFF) as f64
}

fn get_tile_pc(p: Value) -> f64 {
    // TODO ALEX: remove this, get this from config file
    let bosses_in_board: Vec<String> = vec![
        String::from("Zulrah"),
        String::from("butt"),
        String::from("Vetion"),
    ];

    let mut counter = 0;
    // TODO ALEX: please make this a one liner with lambdas or something, functional syntax is clean
    for (key, val) in p["data"].as_object().unwrap() {
        if bosses_in_board.contains(key) && val.as_u64().unwrap() > 0 {
            counter += 1;
        }
    }
    counter as f64 / bosses_in_board.len() as f64 * 100.0
}
