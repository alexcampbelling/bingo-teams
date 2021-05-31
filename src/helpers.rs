use crate::data_structures::{Player, Team};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
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

// TODO: use generics for this printing structure function
pub fn write_csv_player(players: &Vec<Player>, path: &str) -> Result<(), Box<dyn Error>> {
    // Build CSV writer with given path as file name
    let mut writer = csv::Writer::from_path(Path::new(path))?;
    for player in players {
        writer.serialize(player)?;
    }

    // Writer keeps an internal buffer, remember to flush! (Why doesn't this mean unsafe Rust?)
    writer.flush()?;
    Ok(())
}

// TODO ALEX: remove this when making generic printing structure function
pub fn write_csv_teams(teams: &Vec<Team>, path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(Path::new(path))?;
    let mut file = LineWriter::new(file);
    for team in teams {
        file.write_all(b"new team\n")?;
        for player in &team.members {
            file.write_all(player.name.as_bytes())?;
            file.write_all(b", ")?;
        }
        file.write_all(format!("team score: {}", team.team_score).as_bytes())?;
        file.write_all(format!("team size: {}", team.members.len()).as_bytes())?;
        file.write_all(b"\n")?;
    }

    file.flush()?;

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

    Ok(players)
}
