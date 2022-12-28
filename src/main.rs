extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use glutin_window::GlutinWindow as Window;
use graphics::CharacterCache;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};

use piston::event_loop::{Events, EventSettings};
use piston::input::{ButtonState, Button, Key};
use piston::{ButtonEvent, RenderArgs, RenderEvent, UpdateEvent, WindowSettings};

use rand::Rng;

use tetris::libs::constants::app_constants::*;
use tetris::libs::map_src::map::*;
use tetris::libs::pieces_src::pieces::*;

/*
TODO:
   INSTA-DROP
   PAUSE/START
 */

fn set_hold_piece(mut current_piece: Piece, mut hold_piece: Piece, next_piece: &Piece) -> (Piece, Piece, bool) {
    let mut need_new_piece = false;

    if hold_piece.piece != PieceType::None {
        let hold_piece_ref = hold_piece;

        hold_piece = Piece::new(current_piece.piece, [1, 1]);
        current_piece = Piece::new(hold_piece_ref.piece, current_piece.position);

        current_piece.check_x();

    } else {
        need_new_piece = true;

        hold_piece = Piece::new(current_piece.piece, [1, 1]);

        current_piece = *next_piece;
    }

    (current_piece, hold_piece, need_new_piece)
}

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

    let new_piece = Piece::new(piece, [4, 0]);

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

fn draw_game(gl: &mut GlGraphics, piece: &Piece, map: &mut Map, r: RenderArgs, next_piece: &Piece, hold_piece: &Piece, score: i64 ) {
    let offset = SCREEN_SIZE[0]/4.0;
    let mut hold_piece_map = [[MapCell::new(PieceType::None); 4]; 4];
    let mut next_piece_map = [[MapCell::new(PieceType::None); 4]; 4];
    let next_piece_ref = match next_piece.piece {
        PieceType::I => {
            Piece::new(next_piece.piece, [2,1])
        }
        _ => Piece::new(next_piece.piece, [1,1])
    };
    let hold_piece_ref = match hold_piece.piece {
        PieceType::I => {
            Piece::new(hold_piece.piece, [2,1])
        }
        _ => Piece::new(hold_piece.piece, [1,1])
    };

    let ref mut chars = GlyphCache::new("fonts.ttf", (), TextureSettings::new())
        .expect("Could not load font");
    let score_str = score.to_string();

    gl.draw(r.viewport(), |c, g| {
        graphics::clear(WHITE, g);

        for i in 0..4 {
            for j in 0..4 {
                let hold_piece_box = graphics::rectangle::square((PIXEL_SIZE * i as f64 ) + 10.0, (PIXEL_SIZE * j as f64) + 5.0, PIXEL_SIZE);

                if hold_piece_ref.piece !=PieceType::None {
                    if hold_piece_ref.shape.contains(&[i, j]) {
                        hold_piece_map[i as usize][j as usize] = MapCell::new(hold_piece_ref.piece);
                    }
                }

                let color = hold_piece_map[i as usize][j as usize].color;

                graphics::Rectangle::new_border(BLACK, 2.0).color(color).draw(
                    hold_piece_box,
                    &c.draw_state,
                    c.transform,
                    g
                )
            }
        }

        use graphics::Transformed;
        graphics::Text::new_color(BLACK, 32).draw(
            score_str.as_str(),
            chars,
            &c.draw_state,
            c.transform.trans(PIXEL_SIZE, (PIXEL_SIZE * 5.0) + 10.0),
            g,
        ).expect("Error with text");

        for i in 0..WORLD_SIZE[0] as i32 {
            for j in 0..WORLD_SIZE[1] as i32 {
                let cell = graphics::rectangle::square((PIXEL_SIZE * i as f64) + offset, (PIXEL_SIZE * j as f64) + 5.0, PIXEL_SIZE);

                map_check(&piece, map);

                let color = map[i as usize][j as usize].color;

                graphics::Rectangle::new_border(BLACK, 2.0).color(color).draw(
                    cell,
                    &c.draw_state,
                    c.transform,
                    g
                )
            }
        }


        for i in 0..4 {
            for j in 0..4 {
                let next_piece_box = graphics::rectangle::square((PIXEL_SIZE * (i as f64 + WORLD_SIZE[0])) + offset + 10.0, (PIXEL_SIZE * j as f64) + 5.0, PIXEL_SIZE);

                if next_piece_ref.shape.contains(&[i, j]) {
                    next_piece_map[i as usize][j as usize] = MapCell::new(next_piece_ref.piece);
                }

                let color = next_piece_map[i as usize][j as usize].color;

                graphics::Rectangle::new_border(BLACK, 2.0).color(color).draw(
                    next_piece_box,
                    &c.draw_state,
                    c.transform,
                    g
                )
            }
        }
    })
}

fn game_over_screen(gl: &mut GlGraphics, r: RenderArgs) {

    let lose_str = "You Lost";


    let ref mut glyphs = GlyphCache::new("fonts.ttf", (), TextureSettings::new())
        .expect("Could not load font");

    let mut width = 0.0;
    for ch in lose_str.chars() {
        let character = glyphs.character(64, ch).ok().unwrap();
        width += (character.advance_width() + character.left()) as f64;
    }

    let screen_size = SCREEN_SIZE[0];

    gl.draw(r.viewport(), |c, g| {
        graphics::clear(WHITE, g);

        use graphics::Transformed;
        graphics::Text::new_color(BLACK, 64).draw(
            lose_str,
            glyphs,
            &c.draw_state,
            c.transform.trans((screen_size - width)/2.0, START_SIZE[1]/2.0),
                g
        ).expect("Error Loading Text!");

    });
}

fn main() {

    let opengl = OpenGL::V4_5;

    let settings = WindowSettings::new("Tetris", SCREEN_SIZE).exit_on_esc(true);
    let mut window: Window = settings.build().expect("Error Creating Window");

    let mut gl = GlGraphics::new(opengl);

    let mut map = build_map();

    let (mut piece, mut game_over) = rand_piece(&map);
    let (mut next_piece, _holder) = rand_piece(&map);
    let mut hold_piece = Piece::null_piece();

    let mut new_piece: bool;
    let mut score: i64 = 0;

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
                draw_game(&mut gl, &piece, &mut map, r, &next_piece, &hold_piece, score);
            } else {
                game_over_screen(&mut gl, r);
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                if !game_over {
                    match k.button {
                        Button::Keyboard(Key::S) | Button::Keyboard(Key::Down) | Button::Keyboard(Key::Space) => {
                            keys.insert(k.button);
                        },
                        Button::Keyboard(Key::W) | Button::Keyboard(Key::Up) | Button::Keyboard(Key::P) | Button::Keyboard(Key::X) => {
                            piece.rotate("Clockwise", &mut map);
                        },
                        Button::Keyboard(Key::O) | Button::Keyboard(Key::Z) => {
                            piece.rotate("Counter", &mut map);
                        },
                        Button::Keyboard(Key::A) | Button::Keyboard(Key::Left) => {
                            piece.shift("left", &mut map);
                        },
                        Button::Keyboard(Key::D) | Button::Keyboard(Key::Right) => {
                            piece.shift("right", &mut map);
                        },
                        Button::Keyboard(Key::Q) | Button::Keyboard(Key::LShift) | Button::Keyboard(Key::RShift) => {
                            (piece, hold_piece, new_piece) = set_hold_piece(piece, hold_piece, &next_piece);

                            if new_piece {
                                (next_piece, game_over) = rand_piece(&map);
                            }
                        }
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

            let mut loops = true;
            for key in &keys {
                if loops {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    match key {
                        Button::Keyboard(Key::Space) | Button::Keyboard(Key::Down) | Button::Keyboard(Key::S) => {
                            piece.fall(&mut map);
                            piece.check_x();
                        },
                        _ => {},
                    }
                    for cord in piece.shape {
                        if cord[1] == WORLD_SIZE[1] as i32 - 1 {
                            loops = false;
                        }
                    }
                }
            }

            let landed = piece.check_landed(&mut map);
            if seconds.floor() > old_seconds.floor() {
                if !landed {
                    piece.fall(&mut map);
                } else {
                    score += &(piece.landed(&mut map) as i64);
                }
            }

            if !piece.active {
                piece = next_piece;
                (next_piece, game_over) = rand_piece(&map);
            }

            old_seconds = seconds;
        }
    }
}
