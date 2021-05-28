use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Player {
    pub name: String,
    pub ehp: f64,
    pub ehb: f64,
    pub ehp_avg: f64,
    pub ehb_avg: f64,
    pub slayer_ability: f64,
    pub tiles_score: f64,
    pub manual_score: u64,
    pub weighted_score: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Team {
    pub team_number: u64,
    pub members: Vec<Player>,
    pub team_score: u64,
}
