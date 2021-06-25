use crate::data_structures::{Player, Team};

pub fn sort_player_teams(players: &mut Vec<Player>, team_count: u64) -> Vec<Team> {
    let mut teams: Vec<Team> = Vec::new();
    for i in 0..team_count {
        teams.push(Team {
            ..Default::default()
        });
    }
    println!("team count: {:?}", team_count);

    // Sort teams
    players.sort_by(|a, b| b.weighted_score.cmp(&a.weighted_score));

    let mut player_iter = players.clone().into_iter();

    let mut direction = true;
    let mut counter = 0;
    for _i in 0..players.len() {
        // let mut player = player_iter.next().unwrap();
        // println!("checking, got player: {:?}", player.name);
        // println!("checking their score: {:?}", player.weighted_score);
        // println!("Team {:?} is getting player: {:?}", counter, player.name);

        match player_iter.next() {
            Some(player_op) => {
                teams[counter].members.push(player_op.clone());
                teams[counter].team_number = counter as u64 + 1;
                teams[counter].team_score += player_op.weighted_score;
            }
            _ => return teams,
        }

        if ((counter == (team_count - 1) as usize) && (direction == true))
            || ((counter == 0) && (direction == false))
        {
            // match player_iter.next() {
            //     Some(player_op) => {
            //         teams[counter].members.push(player_op.clone());
            //         teams[counter].team_number = counter as u64 + 1;
            //         teams[counter].team_score += player_op.weighted_score;
            //         direction = !direction;
            //     }
            //     _ => return teams,
            // }
            direction = !direction;
            continue;
        }
        if direction {
            counter += 1;
        } else {
            counter -= 1;
        }
    }

    teams
}
