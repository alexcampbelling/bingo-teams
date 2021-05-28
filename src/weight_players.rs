use crate::data_structures::Player;

// TODO ALEX: move this to a config file
const EHP_T_W: f64 = 0.15;
const EHB_T_W: f64 = 0.15;
const EHP_A_W: f64 = 0.10;
const EHB_A_W: f64 = 0.10;
const TILE_W: f64 = 0.25;
const SLAYER_W: f64 = 0.25;

// TODO ALEX: complete this
pub fn weight_scores(players: &mut Vec<Player>) -> &mut Vec<Player> {
    for player in players.iter_mut() {
        let weighted_score: f64 = weight_calculator(player);
        player.weighted_score = weighted_score.round() as u64;
    }

    players
}

fn weight_calculator(player: &Player) -> f64 {
    let score: f64 = player.ehp / 100.0 * EHP_T_W
        + player.ehb / 100.0 * EHB_T_W
        + player.tiles_score / 10.0 * TILE_W
        + player.slayer_ability / 10.0 * SLAYER_W
        + player.ehp_avg * EHP_A_W
        + player.ehb_avg * EHB_A_W
        + player.manual_score as f64;
    score
}
