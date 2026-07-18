use crate::config::{CRATE, CRATE_GOAL, FLOOR, GOAL, PLAYER, PLAYER_GOAL};

#[derive(Clone)]
pub struct GameState {
    pub map: Vec<Vec<u8>>,
    pub player: (usize, usize),
    pub direction: (i32, i32),
}

pub fn walkable(tile: u8) -> bool {
    tile == FLOOR || tile == GOAL
}

pub fn move_player(
    map: &mut Vec<Vec<u8>>,
    player: &mut (usize, usize),
    direction: (i32, i32),
) -> bool {
    let nx = player.0 as i32 + direction.0;
    let ny = player.1 as i32 + direction.1;

    if nx < 0 || ny < 0 {
        return false;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    if ny >= map.len() || nx >= map[0].len() {
        return false;
    }

    let target = map[ny][nx];

    if walkable(target) {
        let old = map[player.1][player.0];

        map[player.1][player.0] = if old == PLAYER_GOAL { GOAL } else { FLOOR };
        map[ny][nx] = if target == GOAL { PLAYER_GOAL } else { PLAYER };

        *player = (nx, ny);
        return true;
    }

    if target == CRATE || target == CRATE_GOAL {
        let bx = nx as i32 + direction.0;
        let by = ny as i32 + direction.1;

        if bx < 0 || by < 0 {
            return false;
        }

        let bx = bx as usize;
        let by = by as usize;

        let behind = map[by][bx];

        if walkable(behind) {
            map[by][bx] = if behind == GOAL { CRATE_GOAL } else { CRATE };
            map[ny][nx] = if target == CRATE_GOAL { PLAYER_GOAL } else { PLAYER };

            let old = map[player.1][player.0];
            map[player.1][player.0] = if old == PLAYER_GOAL { GOAL } else { FLOOR };

            *player = (nx, ny);
            return true;
        }
    }

    false
}

pub fn check_win(map: &Vec<Vec<u8>>) -> bool {
    for row in map {
        for tile in row {
            if *tile == CRATE {
                return false;
            }
        }
    }
    true
}
