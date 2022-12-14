extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{ButtonState, Button, Key};
use piston::{ButtonEvent, RenderEvent, WindowSettings};

use rand::Rng;

use tetris::libs::constants::app_constants::*;
use tetris::libs::map_src::map::*;
use tetris::libs::pieces_src::pieces::*;


fn rand_piece() -> Piece {
    let rand_num = rand::thread_rng().gen_range(1..=7);

    let piece = match rand_num {
        1 => PieceType::I,
        2 => PieceType::J,
        3 => PieceType::L,
        4 => PieceType::O,
        5 => PieceType::S,
        6 =>PieceType::T,
        7 => PieceType::Z,
        _ => PieceType::None,
    };

    Piece::new(piece)
}

fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", START_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);
    let mut _map = build_map();
    let mut start_piece = rand_piece();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(WHITE, g);

                for i in 0..WORLD_SIZE[0] as i32 {
                    for j in 0..WORLD_SIZE[1] as i32 {
                        let mut color = WHITE;
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];

                        if start_piece.shape.contains(&[i, j]) {
                            color = start_piece.color;
                        }

                        graphics::Rectangle::new_border(BLACK, 2.0).color(color).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }
            })
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Space) => {
                        start_piece.fall();
                    },
                    Button::Keyboard(Key::S) => {
                        start_piece.rotate("Counter");
                    },
                    Button::Keyboard(Key::W) => {
                        start_piece.rotate("Clockwise");
                    }
                    _ => {},
                }
            }
        }
    }
}
