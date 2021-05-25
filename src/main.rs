extern crate reqwest;
extern crate tokio;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::io;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Player {
    name: String,
    slayer: Option<u64>,
    ehp: Option<u64>,
    ehb: Option<u64>,
    tiles_score: Option<u64>,
    ehp_avg: Option<u64>,
    ehb_avg: Option<u64>,
    manual_score: Option<u64>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO ALEX: add flags for start up for different functionality
    // cargo run src/main.rs < input/usernames_test.csv

    // Read in list of usernames from csv (stdin)
    let players = read_usernames();

    // TODO ALEX: Call temple to update players?

    // Check temple osrs can be called
    let players: Vec<Player> = create_players_with_temple(&players).await?;

    // Testing changing values
    for p in players.iter() {
        println!(
            "{} | Slayer: {:?}, EHP: {:?}, EHB: {:?}",
            p.name, p.slayer, p.ehp, p.ehb
        )
    }

    Ok(())
}

async fn create_players_with_temple(player_names: &Vec<String>) -> Result<Vec<Player>, Error> {
    let mut players = Vec::new();

    for player in player_names.iter() {
        // Get most current data
        let res_j: Value = reqwest::get(format!(
            "https://templeosrs.com/api/player_stats.php?player={}&bosses=1",
            player
        ))
        .await?
        .json()
        .await?;

        // Get averages data
        // TODO ALEX: this
        // # get ehp and ehb in last THREE months
        // eff_hours_req = req.get(
        //     "https://templeosrs.com/api/player_gains.php?player={}&time={}&bosses=1".format(p_req, DAY_IN_SEC * AVG_DAYS)).json()

        println!("{:?}", res_j["data"]["slayer"]);

        players.push(Player {
            name: player.clone(),
            slayer: res_j["data"]["slayer"].as_u64(),
            ehp: res_j["data"]["Ehp"].as_u64(),
            ehb: res_j["data"]["Ehb"].as_u64(),
            ..Default::default()
        });
    }
    Ok(players)
}

fn tile_score(p: Value) -> u64 {
    let result: u64 = 1;
    result
}

#[allow(dead_code)]
fn change_values(players: &mut Vec<Player>) {
    // quickly checking if i can alter those player structs in here
    for player in players.iter_mut() {
        player.slayer = Some(11);
    }
}

fn read_usernames() -> Vec<String> {
    let mut players: Vec<String> = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we assume fine, take ok and unwrap
        let record = result.ok().unwrap();
        players.push(record[0].parse().unwrap());
    }
    players
}
