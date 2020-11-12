extern crate opengl_graphics;
extern crate piston_window;

use crate::visual::visu::Visu;

use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::*;
use std::panic;

pub fn start_visual(board_array: &[Vec<i32>], size: i32, time: String, heuristic: String) {
    let mut index: usize = 0;
    let opengl = OpenGL::V3_2;

    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    let result = panic::catch_unwind(|| {
        WindowSettings::new("npuzzle", [500, 835])
            .graphics_api(opengl)
            .fullscreen(false)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap()
    });

    let mut window: PistonWindow = match result {
        Ok(window) => window,
        Err(_e) => {
            println!("There was an error while creating the window");
            panic!("There was an error while creating the window");
        }
    };

    let mut visu = Visu {
        gl: GlGraphics::new(opengl),
        board: board_array[index].clone(),
        size: size,
        time: time,
        margin_top: 110.0,
        margin_x: 10.0,
        number_scale: 5.0,
        index: 0,
        total_moves: board_array.len() as i32 - 1,
        heuristic: heuristic,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            visu.render(&args);
        }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::Right) => {
                    if index < board_array.len() - 1 {
                        index += 1;
                        visu.update_board(&args, board_array[index].clone(), index);
                    }
                }
                Button::Keyboard(Key::Left) => {
                    if index > 0 {
                        index -= 1;
                        visu.update_board(&args, board_array[index].clone(), index);
                    }
                }
                _ => {}
            }
        }
    }
}
