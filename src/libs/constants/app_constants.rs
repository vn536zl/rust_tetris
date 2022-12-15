pub const WHITE: Color = [1.0; 4];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const LIGHT_BLUE: Color = [0.0, 0.94, 0.94, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const ORANGE: Color = [0.94, 0.63, 0.0, 1.0];
pub const PURPLE: Color = [0.63, 0.0, 0.94, 1.0];

pub const START_SIZE: [f64; 2] = [320.0, 640.0];
pub const PIXEL_SIZE: f64 = 32.0;
pub const WORLD_SIZE: [f64; 2] = [START_SIZE[0]/PIXEL_SIZE, START_SIZE[1]/PIXEL_SIZE];

pub type Color = [f32; 4];

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

pub fn get_color(piece: &PieceType) -> Color {
    match piece {
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