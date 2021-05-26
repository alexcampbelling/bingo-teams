mod helpers;

extern crate reqwest;
extern crate tokio;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

const DAY_IN_SEC: i32 = 86400;
const SLAYER_95_XP: i32 = 8771558;
const AVG_DAYS: i32 = 365;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Player {
    name: String,
    ehp: f64,
    ehb: f64,
    ehp_avg: f64,
    ehb_avg: f64,
    slayer: u64,
    tiles_score: f64,
    manual_score: u64,
    weighted_score: f64,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO ALEX: add flags for start up for different functionality
    // cargo run src/main.rs < input/usernames_test.csv

    // Read in list of usernames from csv (stdin)
    let players = helpers::read_usernames();

    // TODO ALEX: Call temple to update players?

    // Check temple osrs can be called
    let players: Vec<Player> = create_players_with_temple(&players).await?;

    // Print structs
    for p in players.iter() {
        println!("{:?}", p)
    }

    helpers::weight_scores(&mut players);

    Ok(())
}

async fn create_players_with_temple(player_names: &Vec<String>) -> Result<Vec<Player>, Error> {
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

        let res_avg_j: Value = reqwest::get(format!(
            "https://templeosrs.com/api/player_gains.php?player={}&time={}&bosses=1",
            name,
            DAY_IN_SEC * AVG_DAYS
        ))
        .await?
        .json()
        .await?;

        players.push(Player {
            name: name.clone(),
            slayer: res_j["data"]["Slayer"].as_u64().unwrap(),
            ehp: res_j["data"]["Ehp"].as_f64().unwrap(),
            ehb: res_j["data"]["Ehb"].as_f64().unwrap(),
            ehp_avg: res_avg_j["data"]["Ehp"].as_f64().unwrap(),
            ehb_avg: res_avg_j["data"]["Ehb"].as_f64().unwrap(),
            tiles_score: helpers::tile_score(res_j),
            ..Default::default()
        });
    }
    Ok(players)
}
