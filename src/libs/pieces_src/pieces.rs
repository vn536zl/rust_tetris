use crate::libs::constants::app_constants::*;

pub type PieceShape = [[i32; 3]; 3];

pub enum PieceNames {
    ShapeNone = 0,
    ShapeI = 1,
    ShapeJ = 2,
    ShapeL = 3,
    ShapeO = 4,
    ShapeS = 5,
    ShapeT = 6,
    ShapeZ = 7,
}

pub struct Piece {
    pub piece: PieceNames,
    pub shape: PieceShape,
    pub rotation: i32,
    pub position: [i32; 2],
    pub color: Color,

}