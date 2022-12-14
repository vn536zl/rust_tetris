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


use tetris::libs::constants::app_constants::*;
use tetris::libs::map_src::map::*;


fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", START_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);
    let mut _map = build_map();

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

                        graphics::Rectangle::new_border(BLACK, 2.0).color().draw(
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
                    Button::Keyboard(Key::Space) => {},
                    _ => {},
                }
            }
        }
    }
}
