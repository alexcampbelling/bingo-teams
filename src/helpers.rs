use serde_json::Value;
use std::io;

pub fn tile_score(p: Value) -> f64 {
    let result: f64 = 1.;
    result
}

pub fn read_usernames() -> Vec<String> {
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