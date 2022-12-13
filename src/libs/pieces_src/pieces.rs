use crate::libs::constants::app_constants::*;

pub type PieceShape = [[i32; 2]; 4];

#[derive(Clone)]
pub struct Piece {
    pub shape_id: i32,
    pub shape: PieceShape,
    pub color: Color,
    pub rotation: i32,
    pub active: bool,
}

impl Piece {
    pub fn new(shape_id: i32) -> Self {
        Piece {
            shape_id,
            shape: draw_pieces(shape_id,[4, 0]),
            color: match shape_id {
                1 => LIGHT_BLUE,
                2 => BLUE,
                3 => ORANGE,
                4 => YELLOW,
                5 => GREEN,
                6 => PURPLE,
                7 => RED,
                0 => WHITE,
                _ => BLACK,
            },
            rotation: 0,
            active: match shape_id {
                0 => false,
                _ => true
            },
        }
    }

    pub fn fall(&mut self, map: &Map) {
        let mut new_shape: [[i32; 2]; 4] = [[0, 0]; 4];

        if self.active {
            let mut counter = 0;
            for cord in self.shape {
                if (cord[1] + 1 < WORLD_SIZE[1] as i32) && (map[cord[0] as usize][(cord[1] + 1) as usize] != 2) {
                    new_shape[counter] = [cord[0], cord[1] + 1];
                } else {
                    self.landed();
                }
                counter += 1;
            }
        }

        if new_shape != [[0, 0]; 4] {
            self.shape = new_shape;
        }
    }

    pub fn landed(&mut self) {
        self.active = false;
    }

    pub fn rotate(&mut self, dir: &str) {

        let shape = self.shape;
        if dir == "left" {
            let (point1, point2, point3, point4) = (shape[0], shape[1], shape[2], shape[3]);
            self.shape = match self.shape_id {
                1 => {
                    [
                        [point1[0]+1, point1[1]+1],
                        point2,
                        [point3[0]-1, point3[1]-1],
                        [point4[0]-2, point4[0]-2],
                    ]
                },
                2 => {
                    [
                        [point1[0]+1, point1[1]],
                        point2,
                        [point3[0]-1, point3[1]+1],
                        [point3[0]-2, point3[1]+1],
                    ]
                }
                _ => [
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                ]
            }
        } else if dir == "right" {
            if self.shape_id != 4 {

            }
        }
    }
}

fn draw_pieces(shape: i32, pos: [i32; 2]) -> PieceShape {
    let area: PieceShape = match shape {
        1 => {[
            [pos[0]-1, pos[1]],
            [pos[0], pos[1]],
            [pos[0]+1, pos[1]],
            [pos[0]+2, pos[1]],
        ]},
        2 => {[
            [pos[0], pos[1]+1],
            [pos[0], pos[1]],
            [pos[0]+1, pos[1]],
            [pos[0]+2, pos[1]],
        ]},
        3 => {[
            [pos[0]-1, pos[1]],
            [pos[0], pos[1]],
            [pos[0]+1, pos[1]],
            [pos[0]+1, pos[1]-1],
        ]},
        4 => {[
            [pos[0], pos[1]+1],
            [pos[0], pos[1]],
            [pos[0]+1, pos[1]],
            [pos[0]+1, pos[1]+1],
        ]},
        5 => {[
            [pos[0]-1, pos[1]+1],
            [pos[0], pos[1]],
            [pos[0], pos[1]+1],
            [pos[0]+1, pos[1]]
        ]},
        6 => {[
            [pos[0]-1, pos[1]+1],
            [pos[0], pos[1]],
            [pos[0], pos[1]+1],
            [pos[0]+1, pos[1]+1],
        ]},
        7 => {[
            [pos[0]-1, pos[1]],
            [pos[0], pos[1]],
            [pos[0], pos[1]+1],
            [pos[0]+1, pos[1]+1]
        ]},
        _ => {[
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0]
        ]},
    };

    area
}