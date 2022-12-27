extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};

use piston::event_loop::{Events, EventSettings};
use piston::input::{ButtonState, Button, Key};
use piston::{ButtonEvent, RenderArgs, RenderEvent, UpdateEvent, WindowSettings};

use rand::Rng;

use tetris::libs::constants::app_constants::*;
use tetris::libs::map_src::map::*;
use tetris::libs::pieces_src::pieces::*;

fn rand_piece(map: &Map) -> (Piece, bool) {
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

    let new_piece = Piece::new(piece);

    let mut lose = false;
    for cord in new_piece.shape {
        if map[cord[0] as usize][cord[1] as usize].filled {
            lose = true;
        }
    }

    (new_piece, lose)
}

fn map_check(piece: &Piece, map: &mut Map) {

    for i in 0..WORLD_SIZE[0] as i32 {
        for j in 0..WORLD_SIZE[1] as i32 {
            if !map[i as usize][j as usize].filled {
                map[i as usize][j as usize] = MapCell::new(PieceType::None);
            }

            if piece.shape.contains(&[i, j]) {
                map[i as usize][j as usize] = MapCell::new(piece.piece);
            }
        }
    }
}

fn draw_game(gl: &mut GlGraphics, piece: &Piece, map: &mut Map, r: RenderArgs ) {
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

                map_check(&piece, map);

                let color = map[i as usize][j as usize].color;

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

fn game_over_screen(gl: &mut GlGraphics, r: RenderArgs) {

    let lose_str = "You Lost";


    let ref mut glyphs = GlyphCache::new("fonts.ttf", (), TextureSettings::new())
        .expect("Could not load font");

    gl.draw(r.viewport(), |c, g| {
        graphics::clear(WHITE, g);

        use graphics::Transformed;
        graphics::Text::new_color(BLACK, 64).draw(
            lose_str,
            glyphs,
            &c.draw_state,
            c.transform.trans(START_SIZE[0]/8.0, START_SIZE[1]/2.0),
                g
        ).expect("Error Loading Text!");

    });
}

fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", START_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);

    let mut map = build_map();
    let (mut piece, mut game_over) = rand_piece(&map);

    let mut keys = HashSet::new();
    let mut seconds: f64 = 0.0;
    let mut old_seconds: f64 = 0.0;


    let font_file_bytes = include_bytes!("font.ttf");
    let mut font_file = File::create("fonts.ttf").expect("Could Not Create File");
    font_file.write_all(font_file_bytes).expect("Could Not Write to file");


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            if !game_over {
                draw_game(&mut gl, &piece, &mut map, r);
            } else {
                game_over_screen(&mut gl, r);
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                if !game_over {
                    match k.button {
                        Button::Keyboard(Key::S) => {
                            keys.insert(k.button);
                        },
                        Button::Keyboard(Key::P) => {
                            piece.rotate("Clockwise", &mut map);
                        },
                        Button::Keyboard(Key::O) => {
                            piece.rotate("Counter", &mut map);
                        },
                        Button::Keyboard(Key::A) => {
                            piece.shift("left", &mut map);
                        },
                        Button::Keyboard(Key::D) => {
                            piece.shift("right", &mut map);
                        },
                        Button::Keyboard(Key::Down) => {
                            keys.insert(k.button);
                        },
                        Button::Keyboard(Key::Up) => {
                            piece.rotate("Clockwise", &mut map);
                        },
                        Button::Keyboard(Key::Left) => {
                            piece.shift("left", &mut map);
                        },
                        Button::Keyboard(Key::Right) => {
                            piece.shift("right", &mut map);
                        },
                        Button::Keyboard(Key::Space) => {
                            keys.insert(k.button);
                        },
                        Button::Keyboard(Key::X) => {
                            piece.rotate("Clockwise", &mut map);
                        },
                        Button::Keyboard(Key::Z) => {
                            piece.rotate("Counter", &mut map);
                        },
                        _ => {},
                    }
                } else {
                    match k.button {
                        Button::Keyboard(Key::R) => {
                            map = build_map();
                            (piece, game_over) = rand_piece(&map);
                        }
                        _ => {}
                    }
                }
            }
            if k.state == ButtonState::Release {
                keys.remove(&k.button);
            }
        }
        if let Some(u) = e.update_args() {
            seconds += u.dt;

            for key in &keys {
                std::thread::sleep(std::time::Duration::from_millis(50));
                match key {
                    Button::Keyboard(Key::Space) => {
                        piece.fall(&mut map);
                    },
                    Button::Keyboard(Key::Down) => {
                        piece.fall(&mut map);
                    },
                    Button::Keyboard(Key::S) => {
                        piece.fall(&mut map);
                    },
                    _ => {},
                }
            }

            let landed = piece.check_landed(&mut map);
            if seconds.floor() > old_seconds.floor() {
                if !landed {
                    piece.fall(&mut map);
                } else {
                    piece.landed(&mut map);
                }
            }

            if !piece.active {
                (piece, game_over) = rand_piece(&map);
            }

            old_seconds = seconds;
        }
    }
}
