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

use rand::Rng;

use tetris::libs::constants::app_constants::*;
use tetris::libs::pieces_src::pieces::Piece;
use tetris::libs::map_src::map::*;



fn random_piece() -> Piece {
    let rand = rand::thread_rng().gen_range(1..=7);

    Piece::new(rand)
}

fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", START_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);
    let mut map = build_map();

    let mut active_piece = random_piece();

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

                        if active_piece.shape.contains(&[i, j]) {
                            color = active_piece.color;
                            map[i as usize][j as usize] = 1;
                        }else if map[i as usize][j as usize] != 2 {
                            map[i as usize][j as usize] = 0;
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
                    Button::Keyboard(Key::S) => {
                        active_piece.fall(&map)
                    },
                    Button::Keyboard(Key::W) => {
                        active_piece.fall(&map)
                    },
                    Button::Keyboard(Key::A) => {
                        active_piece.fall(&map)
                    },
                    Button::Keyboard(Key::D) => {
                        active_piece.fall(&map)
                    },
                    _ => {},
                }
            }
        }
    }
}
