use crate::libs::constants::app_constants::*;
use std::collections::HashMap;
use itertools::Itertools;

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

    let mut y_filled: HashMap<i32, i32> = HashMap::new();

    for i in 0..WORLD_SIZE[0] as i32 {
        for j in 0..WORLD_SIZE[1] as i32 {
            if map[i as usize][j as usize].filled {
                let old_val = y_filled.get(&j).copied().unwrap_or(0);
                y_filled.insert(j, old_val + 1);
            }
        }
    }


    for key in y_filled.keys().sorted().copied() {
        let val = y_filled.get(&key).copied().unwrap();
        if val == WORLD_SIZE[0] as i32 {
            for i in 0..WORLD_SIZE[0] as i32 {
                for mut j in 0..(key+1) {
                    println!("{}", j);
                    j = (j - key).abs();
                    if (j - 1) >= 0 {
                        println!("{:?}", map[i as usize][(j - 1) as usize].piece);
                        map[i as usize][j as usize] = map[i as usize][(j - 1) as usize]
                    } else {
                        map[i as usize][j as usize] = MapCell::new(PieceType::None);
                    }
                }
            }
        }
    }
}