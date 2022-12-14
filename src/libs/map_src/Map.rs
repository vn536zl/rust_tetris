use crate::libs::constants::app_constants::*;

pub fn build_map() -> Map {
    let map: Map = vec![vec![0; WORLD_SIZE[1] as usize]; WORLD_SIZE[0] as usize];
    map
}