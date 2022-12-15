use piston::Key::W;
use crate::libs::constants::app_constants::*;

#[derive(Copy, Clone)]
pub struct MapCell {
    pub filled: bool,
    pub color: Color,
    pub piece: PieceType,
}

pub type Map = Vec<Vec<MapCell>>;

impl MapCell {
    pub fn new(piece: PieceType) -> Self {
        MapCell {
            filled: false,
            color: get_color(&piece),
            piece,
        }
    }
}

pub fn build_map() -> Map {
    let map: Map = vec![vec![MapCell::new(PieceType::None); WORLD_SIZE[1] as usize]; WORLD_SIZE[0] as usize];
    map
}

pub fn check_lines(map: &mut Map) {

    let mut not_full = true;

    for j in (WORLD_SIZE[1]-1.0) as i32..=0 {
        let mut line = 0;
        for i in 0..WORLD_SIZE[0] as i32 {
            if map[i as usize][j as usize].filled {
                if not_full {
                    line += 1;
                    if line == WORLD_SIZE[0] as i32 {
                        not_full = false;
                        break;
                    }
                } else {
                    if (j - 1) >= 0 {
                        map[i as usize][j as usize] = map[i as usize][(j-1) as usize];
                    } else {
                        map[i as usize][j as usize] = MapCell::new(PieceType::None);
                    }
                }
            }
        }
    }
}