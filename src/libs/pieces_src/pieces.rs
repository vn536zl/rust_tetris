use crate::libs::constants::app_constants::*;

pub type PieceShape = [[i32; 2]; 4];

#[derive(Clone)]
pub struct Piece {
    pub shape_id: i32,
    pub shape: PieceShape,
    pub color: Color,
    pub rotation: i32,
    pub active: bool,
    pub pos: [i32; 2],
}

impl Piece {
    pub fn new(shape_id: i32) -> Self {
        Piece {
            shape_id,
            shape: draw_pieces(shape_id, 0, [4, 0]),
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
            pos: [4, 0],
        }
    }

    pub fn fall(&mut self, map: &Map) {
        let mut new_shape: [[i32; 2]; 4] = [[0, 0]; 4];

        if self.active {
            let mut counter = 0;
            for cord in self.shape {
                if (cord[1] + 1 < WORLD_SIZE[1] as i32 - 1) && (map[cord[0] as usize][(cord[1] + 1) as usize] != 2) {
                    new_shape[counter] = [cord[0], cord[1] + 1];
                } else {
                    self.landed();
                }
                counter += 1;
            }
        }

        if new_shape != [[0, 0]; 4] {
            self.shape = new_shape;
            self.pos = [self.pos[0], self.pos[1]+1]
        }
    }

    pub fn landed(&mut self) {
        self.active = false;
    }

    pub fn rotate(&mut self, dir: &str) {
        let mut rotation = self.rotation;
        if dir == "left" {
            rotation -= 1;
            if rotation < 0 {
                rotation = 3;
            }
        } else if dir == "right" {
            rotation += 1;
            if rotation > 3 {
                rotation = 0;
            }
        }

        self.rotation = rotation;
        self.shape = draw_pieces(self.shape_id, self.rotation, self.pos);
    }
}

fn draw_pieces(shape_id: i32, rotation: i32, pos: [i32; 2]) -> PieceShape {
    let area: PieceShape = match shape_id {
        1 => {
            match rotation {
                0 => {[
                    [pos[0]-1, pos[1]],
                    [pos[0], pos[1]],
                    [pos[0]+1, pos[1]],
                    [pos[0]+2, pos[1]],
                ]},
                1 => {[
                    [pos[0], pos[1]-1],
                    [pos[0], pos[1]],
                    [pos[0], pos[1]+1],
                    [pos[0], pos[1]+2],
                ]}
                2 => {[
                    [pos[0]-1, pos[1]],
                    [pos[0], pos[1]],
                    [pos[0]+1, pos[1]],
                    [pos[0]+2, pos[1]],
                ]},
                3 => {[
                    [pos[0], pos[1]-1],
                    [pos[0], pos[1]],
                    [pos[0], pos[1]+1],
                    [pos[0], pos[1]+2],
                ]}
                _ => {
                    println!("Rotation Out of bounds");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        2 => {
            match rotation {
                0=> {[
                    [pos[0]-1, pos[1]+1],
                    [pos[0]-1, pos[1]],
                    [pos[0], pos[1]],
                    [pos[0]+1, pos[1]],
                ]},
                1 => {[
                    [pos[0], pos[1]],
                    [pos[0]+1, pos[1]],
                    [pos[0]+1, pos[1]+1],
                    [pos[0]+1, pos[1]+2],
                ]},
                2 => {[
                    [pos[0]+2, pos[1]],
                    [pos[0]+2, pos[1]+1],
                    [pos[0]+1, pos[1]+1],
                    [pos[0], pos[1]+1],
                ]},
                3 => {[
                    [pos[0], pos[1]],
                    [pos[0], pos[1]+1],
                    [pos[0], pos[1]+2],
                    [pos[0]+1, pos[1]+2],
                ]}
                _ => {
                    println!("Rotation Out of bounds");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        3 => {
            match rotation {
                0 => {[
                    [pos[0]-1, pos[1]],
                    [pos[0], pos[1]],
                    [pos[0]+1, pos[1]],
                    [pos[0]+1, pos[1]+1],
                ]},
                1 => {[
                    [pos[0], pos[1]],
                    [pos[0], pos[1]+1],
                    [pos[0], pos[1]+2],
                    [pos[0]-1, pos[1]+2],
                ]}
            }
        },
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