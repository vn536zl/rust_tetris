extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::math::transform_pos;
use graphics::rectangle::Border;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{ButtonState, Button, Key};
use piston::{ButtonEvent, RenderEvent, WindowSettings};
use piston::Key::B;

type Color = [f32; 4];

const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];

const START_SIZE: [f64; 2] = [320.0, 512.0];
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: [f64; 2] = [START_SIZE[0]/PIXEL_SIZE, START_SIZE[1]/PIXEL_SIZE];

#[derive(Clone)]
struct Piece {
    shape: i32,
    rotation: f64,
    pos: [i32; 2],
}

impl Piece {
    fn new(shape: i32) -> Self {
        Piece {
            shape,
            rotation: 0.0,
            pos: [5, 0],
        }
    }

    fn rotate(&mut self, dir: i32) {
        if dir == 0 {
            self.rotation -= 2.0;
        } else {
            self.rotation += 2.0;
        }
    }
}

type Map = Vec<Vec<Piece>>;

fn build_map() -> Map {
    let mut map: Map = vec![vec![Piece::new(0); WORLD_SIZE[1] as usize]; WORLD_SIZE[0] as usize];

    map[1][2] = Piece::new(4);

    map
}

fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", START_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);
    let map = build_map();

    println!("{}, {}", map.len(), map[0].len());

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(WHITE, g);

                for i in 0..WORLD_SIZE[0] as i32 {
                    for j in 0..WORLD_SIZE[1] as i32 {
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];

                        graphics::Rectangle::new_border(BLACK, 2.0).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );

                        if map[i as usize][j as usize].shape != 0 {
                            graphics::Rectangle::new_border(BLACK, 2.0).color(YELLOW).draw(
                                pos,
                                &c.draw_state,
                                c.transform,
                                g,
                            );
                        }
                    }
                }
            })
        }
    }
}
