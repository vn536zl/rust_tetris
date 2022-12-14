use crate::libs::constants::app_constants::*;

pub type Shape = [[i32; 2]; 4];

//Relate Shape type to number
#[derive(Clone, Copy)]
pub enum PieceType {
    None,
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

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

fn shape_piece(piece: &PieceType, pos: [i32; 2], rotation: i32) -> Shape {
    let base_shape = match piece {
        PieceType::None => {[
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
        ]},
        PieceType::I => {
            match rotation % 2 {
                0 => [
                    [-2, 0],
                    [-1, 0],
                    [0, 0],
                    [1, 0],
                ],
                1 => [
                    [0, -1],
                    [0, -1],
                    [0, 0],
                    [0, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        PieceType::J => {
            match rotation {
                0 => [
                    [-1, 0],
                    [0, 0],
                    [1, 0],
                    [1, 1],
                ],
                1 => [
                    [0, -1],
                    [0, 0],
                    [0, 1],
                    [-1, 1],
                ],
                2 => [
                    [-1, -1],
                    [-1, 0],
                    [0, 0],
                    [1, 0],
                ],
                3 => [
                    [1, 0],
                    [0, -1],
                    [0, 0],
                    [0, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        PieceType::L => {
            match rotation {
                0 => [
                    [-1, 1],
                    [-1, 0],
                    [0, 0],
                    [1, 0],
                ],
                1 => [
                    [-1, -1],
                    [0, -1],
                    [0, 0],
                    [0, 1],
                ],
                2 => [
                    [1, -1],
                    [1, 0],
                    [0, 0],
                    [-1, 0],
                ],
                3 => [
                    [0, -1],
                    [0, 0],
                    [0, 1],
                    [1, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        PieceType::O => {[
            [0, 0],
            [1, 0],
            [0, 1],
            [1, 1],
        ]}
        PieceType::S => {
            match rotation % 2 {
                0 => [
                    [-1, 1],
                    [0, 1],
                    [0, 0],
                    [1, 0],
                ],
                1 => [
                    [-1, -1],
                    [-1, 0],
                    [0, 0],
                    [0, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        PieceType::T => {
            match rotation {
                0 => [
                    [-1, 0],
                    [0, 0],
                    [0, 1],
                    [1, 0],
                ],
                1 => [
                    [0, -1],
                    [-1, 0],
                    [0, 0],
                    [0, 1],
                ],
                2 => [
                    [-1, 0],
                    [0, 0],
                    [0, -1],
                    [1, 0],
                ],
                3 => [
                    [0, -1],
                    [1, 0],
                    [0, 0],
                    [0, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
        PieceType::Z => {
            match rotation % 2 {
                0 => [
                    [-1, 0],
                    [0, 0],
                    [0, 1],
                    [1, 1],
                ],
                1 => [
                    [0, -1],
                    [0, 0],
                    [-1, 0],
                    [-1, 1],
                ],
                _ => {
                    println!("Rotation Error");
                    [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ]
                }
            }
        },
    };

    let mut shape: Shape = [[0; 2]; 4];
    let mut counter = 0;
    for cord in base_shape {
        let new_cords = [(cord[0] + pos[0]), (cord[1] + pos[1])];
        shape[counter] = new_cords;
        counter += 1;
    }

    shape
}

impl Piece {
    pub fn new(piece: PieceType) -> Self {
        Piece {
            piece,
            shape: shape_piece(&piece, [4, 0], 0),
            rotation: 0,
            position: [4, 0], //Center of piece defined at 4, 0
            active: true,
            color: match piece {
                PieceType::None => WHITE,
                PieceType::I => LIGHT_BLUE,
                PieceType::J => BLUE,
                PieceType::L => ORANGE,
                PieceType::O => YELLOW,
                PieceType::S => GREEN,
                PieceType::T => PURPLE,
                PieceType::Z => RED,
            }
        }
    }

    pub fn fall(&mut self) {
        let pos = self.position;
        let new_pos = [pos[0], pos[1]+1];


        self.shape = shape_piece(&self.piece, new_pos, self.rotation);
        self.position = new_pos;

    }

    pub fn rotate(&mut self, dir: &str) {
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
        self.shape = shape_piece(&self.piece, self.position, self.rotation);
    }
}