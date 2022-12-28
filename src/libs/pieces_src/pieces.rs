use crate::libs::constants::app_constants::*;
use crate::libs::map_src::map::*;

pub type Shape = [[i32; 2]; 4];

//Piece structure
#[derive(Clone, Copy)]
pub struct Piece {
    pub piece: PieceType,
    pub shape: Shape,
    pub active: bool,
    pub rotation: i32,
    pub position: [i32; 2],
    pub color: Color,
}

pub fn shape_piece(piece: &PieceType, pos: [i32; 2], rotation: i32) -> Shape {
    let mut shape: Shape = [[0; 2]; 4];


    let base_shape = match piece {
        PieceType::None => { [[0, 0], [0, 0], [0, 0], [0, 0]] },
        PieceType::I => {
            match rotation % 2 {
                0 => [[-2, 0], [-1, 0], [0, 0], [1, 0]],
                1 => [[0, -2], [0, -1], [0, 0], [0, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                }
            }
        },
        PieceType::J => {
            match rotation {
                0 => [[-1, 0], [0, 0], [1, 0], [1, 1]],
                1 => [[0, -1], [0, 0], [0, 1], [-1, 1]],
                2 => [[-1, -1], [-1, 0], [0, 0], [1, 0]],
                3 => [[1, -1], [0, -1], [0, 0], [0, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                },
            }
        },
        PieceType::L => {
            match rotation {
                0 => [[-1, 1], [-1, 0], [0, 0], [1, 0]],
                1 => [[-1, -1], [0, -1], [0, 0], [0, 1]],
                2 => [[1, -1], [1, 0], [0, 0], [-1, 0]],
                3 => [[0, -1], [0, 0], [0, 1], [1, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                }
            }
        },
        PieceType::O => { [[0, 0], [1, 0], [0, 1], [1, 1]] }
        PieceType::S => {
            match rotation % 2 {
                0 => [[-1, 1], [0, 1], [0, 0], [1, 0]],
                1 => [[-1, -1], [-1, 0], [0, 0], [0, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                }
            }
        },
        PieceType::T => {
            match rotation {
                0 => [[-1, 0], [0, 0], [0, 1], [1, 0]],
                1 => [[0, -1], [-1, 0], [0, 0], [0, 1]],
                2 => [[-1, 0], [0, 0], [0, -1], [1, 0]],
                3 => [[0, -1], [1, 0], [0, 0], [0, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                }
            }
        },
        PieceType::Z => {
            match rotation % 2 {
                0 => [[-1, 0], [0, 0], [0, 1], [1, 1]],
                1 => [[0, -1], [0, 0], [-1, 0], [-1, 1]],
                _ => {
                    println!("Rotation Error");
                    [[0, 0], [0, 0], [0, 0], [0, 0]]
                }
            }
        },
    };

    let mut counter = 0;
    for cord in base_shape {
        let new_cords = [(cord[0] + pos[0]), (cord[1] + pos[1])];
        shape[counter] = new_cords;
        counter += 1;
    }


    shape
}

impl Piece {
    pub fn new(piece: PieceType, pos: [i32; 2]) -> Self {
        Piece {
            piece,
            shape: shape_piece(&piece, pos, 0),
            rotation: 0,
            position: pos,
            active: true,
            color: get_color(&piece)
        }
    }

    pub fn null_piece() -> Self {
        Piece {
            piece: PieceType::None,
            shape: [[0; 2]; 4],
            rotation: 0,
            position: [0, 0],
            active: false,
            color: WHITE,
        }
    }

    pub fn check_landed(&mut self, map: &mut Map) -> bool {
        let mut landed = false;

        for cord in self.shape {
            if cord[1] + 1 >= WORLD_SIZE[1] as i32 {
                landed = true;
                break;
            }
            if cord[1] >= 0 {
                if map[cord[0] as usize][(cord[1] + 1) as usize].filled {
                    landed = true;
                    break;
                }
            }
        }

        landed
    }

    pub fn landed(&mut self, map: &mut Map) -> i32 {
        self.active = false;

        for cord in self.shape {
            if (cord[0] >= 0 && cord[1] >= 0) && (cord[0] < WORLD_SIZE[0] as i32 && cord[1] < WORLD_SIZE[1] as i32) {
                map[cord[0] as usize][cord[1] as usize] = MapCell::new(self.piece);
                map[cord[0] as usize][cord[1] as usize].filled = true;
            }
        }
        check_lines(map)
    }

    pub fn fall(&mut self, map: &mut Map) {

        if self.active {
            let landed = self.check_landed(map);

            if !landed {
                self.position = [self.position[0], self.position[1] + 1];
                self.shape = shape_piece(&self.piece, self.position, self.rotation);
            }
        }
    }

    pub fn rotate(&mut self, dir: &str, map: &mut Map) {

        if dir == "Clockwise" {
            if self.rotation < 3 {
                self.rotation += 1;
            } else {
                self.rotation = 0;
            }
        } else if dir == "Counter" {
            if self.rotation > 0 {
                self.rotation -= 1;
            } else {
                self.rotation = 3;
            }
        }

        let mut new_shape = shape_piece(&self.piece, self.position, self.rotation);
        let mut pos = self.position;

        let mut counter = 0;
        for cord in new_shape {
            if (0 > cord[0]) || (cord[0] >= WORLD_SIZE[0] as i32) {
                break;
            }
            counter += 1;
        }

        if counter < 4 {
            let out_cords = new_shape[counter];

            if out_cords[0] < 0 {
                pos = [self.position[0] + (0 - out_cords[0]), self.position[1]]
            } else if out_cords[0] > WORLD_SIZE[0] as i32 {
                pos = [self.position[0] - (WORLD_SIZE[0] as i32 - out_cords[0]), self.position[1]]
            } else if out_cords[0] == WORLD_SIZE[0] as i32 {
                pos = [self.position[0] - 1, self.position[1]]
            }

            new_shape = shape_piece(&self.piece, pos, self.rotation)
        }

        counter = 0;
        for cord in new_shape {
            if cord[1] >= WORLD_SIZE[1] as i32 {
                break;
            }
            counter += 1;
        }

        if counter < 4 {
            let out_cords = new_shape[counter];

            if out_cords[1] > WORLD_SIZE[1] as i32 {
                pos = [self.position[0], self.position[1] - (WORLD_SIZE[1] as i32 - out_cords[1])]
            } else if out_cords[1] == WORLD_SIZE[1] as i32 {
                pos = [self.position[0], self.position[1] - 1]
            }

            new_shape = shape_piece(&self.piece, pos, self.rotation)
        }

        for cord in new_shape {
            if (0 <= cord[0] && cord[0] < WORLD_SIZE[0] as i32) && (0 <= cord[1] && cord[1] < WORLD_SIZE[1] as i32)  {
                if map[cord[0] as usize][cord[1] as usize].filled {
                    if cord[0] > pos[0] {
                        let dif = cord[0] - pos[0];
                        pos = [pos[0] - dif, pos[1]];
                    }
                    if cord[0] < pos[0] {
                        let dif = pos[0] - cord[0];
                        pos = [pos[0] + dif, pos[1]];
                    }
                    if cord[1] > pos[1] {
                        let dif = cord[1] - pos[1];
                        pos = [pos[0], pos[1] - dif];
                    }
                    if cord[1] < pos[1] {
                        let dif = pos[1] - cord[1];
                        pos = [pos[0], pos[1] - dif];
                    }
                }
            }
        }
        new_shape = shape_piece(&self.piece, pos, self.rotation);

        self.position = pos;
        self.shape = new_shape;

    }

    pub fn shift(&mut self, dir: &str, map: &mut Map) {
        let mut new_pos = [0; 2];
        let mut valid = false;

        if self.active {
            if dir == "left" {
                new_pos = [self.position[0]-1, self.position[1]];
            } else if dir == "right" {
                new_pos = [self.position[0]+1, self.position[1]];
            }

            let new_shape = shape_piece(&self.piece, new_pos, self.rotation);

            for cord in new_shape {
                if (0 > cord[0]) || (cord[0] >= WORLD_SIZE[0] as i32) {
                    valid = false;
                    break;
                } else if (0 <= cord[1] && cord[1] < WORLD_SIZE[1] as i32) && (map[cord[0] as usize][cord[1] as usize].filled) {
                    valid = false;
                    break;
                } else {
                    valid = true;
                }
            }

            if valid {
                self.position = new_pos;
                self.shape = new_shape;
            }
        }
    }

    pub fn check_x(&mut self) {
        let mut dif = 0;

        for cord in self.shape {
            if cord[0] < 0 {
                dif = 0 - cord[0];
            } else if cord[0] >= WORLD_SIZE[0] as i32 {
                dif = WORLD_SIZE[0] as i32 - cord[0];
            }

            self.position = [self.position[0] + dif, self.position[1]];
            self.shape = shape_piece(&self.piece, self.position, self.rotation);
        }
    }
}