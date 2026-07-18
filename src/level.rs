use crate::config::{CRATE, CRATE_GOAL, FLOOR, GOAL, PLAYER, PLAYER_GOAL, WALL};

fn read_bits(bytes: &[u8], bit_cursor: &mut usize) -> Option<u8> {
    if *bit_cursor + 3 > bytes.len() * 8 {
        return None;
    }

    let mut value = 0;

    for _ in 0..3 {
        let byte = bytes[*bit_cursor / 8];
        let bit = (byte >> (7 - (*bit_cursor % 8))) & 1;
        value = (value << 1) | bit;
        *bit_cursor += 1;
    }

    Some(value)
}

pub async fn load_map(id: usize) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let bytes = include_bytes!("../levels.bin");

    let mut byte_cursor = 0;
    let mut level_index = 0;

    while byte_cursor + 2 <= bytes.len() {
        let width = bytes[byte_cursor] as usize;
        let height = bytes[byte_cursor + 1] as usize;
        byte_cursor += 2;

        let mut map = vec![vec![FLOOR; width]; height];
        let mut player = (0, 0);
        let mut bit_cursor = byte_cursor * 8;

        for y in 0..height {
            for x in 0..width {
                let tile = read_bits(bytes, &mut bit_cursor).unwrap();

                map[y][x] = match tile {
                    1 => WALL,
                    2 => FLOOR,
                    3 => GOAL,
                    4 => GOAL,
                    5 => CRATE,
                    6 => {
                        player = (x, y);
                        PLAYER
                    }
                    _ => FLOOR,
                };
            }
        }

        byte_cursor = (bit_cursor + 7) / 8;

        if level_index == id {
            return (map, (width, height), player);
        }

        if byte_cursor < bytes.len() && bytes[byte_cursor] == 0 {
            byte_cursor += 1;
        }

        level_index += 1;
    }

    (Vec::new(), (0, 0), (0, 0))
}
