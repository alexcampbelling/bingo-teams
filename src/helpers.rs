use crate::data_structures::Player;
use std::{error::Error, path::Path};

pub fn read_usernames(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut players: Vec<String> = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(file_name)?;

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we assume fine, take ok and unwrap
        let record = result.ok().unwrap();
        players.push(record[0].parse().unwrap());
    }

    Ok(players)
}

pub fn write_csv(players: &Vec<Player>, path: &str) -> Result<(), Box<dyn Error>> {
    // Build CSV writer with given path as file name
    let mut writer = csv::Writer::from_path(Path::new(path))?;
    for player in players {
        writer.serialize(player)?;
    }

    // Writer keeps an internal buffer, remember to flush! (Why doesn't this mean unsafe Rust?)
    writer.flush()?;
    Ok(())
}

pub fn read_csv_players(file_name: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    // Create vector of Player structures
    let mut players: Vec<Player> = Vec::new();

    // Create CSV reader with given path for input file
    let mut reader = csv::Reader::from_path(Path::new(file_name))?;

    for result in reader.deserialize() {
        let record: Player = result?;
        players.push(record);
    }

    println!("Checking it was read in correctly");
    for player in players.iter() {
        println!("{:?}", player)
    }

    Ok(players)
}
