extern crate reqwest;
extern crate tokio;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, io};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Default)]
struct Player {
    name: String,
    slayer: Option<u64>,
    tiles_score: Option<u64>,
    ehp: Option<u64>,
    ehb: Option<u64>,
    ehp_avg: Option<u64>,
    ehb_avg: Option<u64>,
    manual_score: Option<u64>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO ALEX: add flags for start up for different functionality
    // cargo run src/main.rs < input/usernames_test.csv

    // Read in list of usernames from csv (stdin)
    let mut players = read_usernames();

    // Testing prints
    for player in players.iter() {
        println!("{}", player.name)
    }

    // Testing changing values
    // change_values(&mut players);
    // for player in players.iter_mut() {
    //     println!("{:?}", player.slayer.unwrap())
    // }

    // TODO ALEX: Update all temple osrs from api before getting data -> make sure all up to date
    // Check temple osrs can be called
    update_with_temple(&mut players).await?;
    Ok(())
}

async fn update_with_temple(players: &mut Vec<Player>) -> Result<(), Box<dyn std::error::Error>> {
    // call api and see if there is a response

    // stats_req = req.get(
    //     "https://templeosrs.com/api/player_stats.php?player={}&bosses=1".format(p_req)).json()

    // # get ehp and ehb in last THREE months
    // eff_hours_req = req.get(
    //     "https://templeosrs.com/api/player_gains.php?player={}&time={}&bosses=1".format(p_req, DAY_IN_SEC * AVG_DAYS)).json()

    // TODO ALEX: iterate through players
    // Call temple, wait

    for player in players.iter_mut() {
        let response_string = reqwest::get(format!(
            "https://templeosrs.com/api/player_stats.php?player={}&bosses=1",
            player.name
        ))
        .await?
        .text()
        .await?;

        // Cast response to json
        let res_json: Value = serde_json::from_str(&response_string)?;

        println!("{:?}", res_json["data"]["Slayer"]);

        // Update players structures
        player.slayer = res_json["data"]["Slayer"].as_u64();
        player.tiles_score = tile_score(res_json);
    }

    Ok(())
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

fn read_usernames() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we assume fine, take ok and unwrap
        let record = result.ok().unwrap();

        // Create player struct and append to players list
        let player = Player {
            name: record[0].parse().unwrap(),
            ..Default::default()
        };
        players.push(player)
    }
    players
}
