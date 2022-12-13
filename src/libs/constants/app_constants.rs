pub type Color = [f32; 4];
pub type Map = Vec<Vec<i32>>;

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