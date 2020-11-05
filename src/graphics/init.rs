// extern crate glutin_window;
extern crate piston_window;

// extern crate graphics;
extern crate opengl_graphics;
// extern crate piston;
extern crate find_folder;

use crate::board::utils::*;

use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston_window::*;
// use glutin_window::GlutinWindow as Window;
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,  PressEvent};
// use piston::window::WindowSettings;


pub struct Visu {
    gl: GlGraphics,
    board: Vec<i32>,
    size: i32,
    time: String
}

impl Visu {
    fn render(&mut self, args: &RenderArgs) {
        // use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let size = self.size;

        let grid = grid::Grid {
            cols: size as u32,
            rows: size as u32,
            units: (args.window_size[0]) / size as f64 - 10.0,
        };

        let line = Line::new(RED, 1.0);
        
        let (win_w, win_h) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let board = self.board.clone();
        let time = self.time.clone();
        
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        let ref font = assets.join("font.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            
            let transform = c
                .transform
                .trans(10.0 * size as f64 / 2.0, 10.0 * size as f64 / 2.0);
            
            grid.draw(&line, &c.draw_state, transform, gl);
            let iter = grid.cells();
            for y in 0..size as u32 {
                for x in 0..size as u32 {
                    let pos = grid.cell_position((x, y));
                    let transform = c.transform.trans(pos[0] + (grid.units / 2.0) + 10.0, pos[1] + (grid.units / 2.0) + 10.0);
                    let nb = board[fdtos(x as i32, y as i32, size) as usize];
                    let string: String;
                    if nb == size * size {
                        string = "*".to_string();
                    } else {
                        string = nb.to_string(); 
                    }
                    text::Text::new_color([0.0, 0.5, 0.0, 1.0], 64).draw(
                        &string,
                        &mut glyph_cache,
                        &c.draw_state,
                        transform, gl
                    ).unwrap();
                }
            }

             text::Text::new_color([0.0, 0.5, 0.0, 1.0], 64).draw(
                        &("Time : ".to_string() + &time + &("s".to_string())),
                        &mut glyph_cache,
                        &c.draw_state,
                         c.transform.trans(10.0, 600.0), gl
                    ).unwrap();
        });
    }

    // fn update(&mut self, args: &UpdateArgs) {
    //     // Rotate 2 radians per second.
    // }

    fn update_board(&mut self, args: &Button, board: Vec<i32>) {
        self.board = board;
    }
}

pub fn graphics(board_array: &[Vec<i32>], size: i32, time: String) {
    
    let mut index: usize = 0;
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
                "npuzzle",
                [500, 700]
            )
            .graphics_api(opengl)
            .fullscreen(false)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut visu = Visu {
        gl: GlGraphics::new(opengl),
        board: board_array[index].clone(),
        size: size,
        time: time,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            visu.render(&args);
        }

        // if let Some(args) = e.update_args() {
        //     visu.update(&args);
        // }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::Right) => {
                    if index < board_array.len() - 1 {
                        index += 1;
                        visu.update_board(&args, board_array[index].clone());
                    }
                },
                Button::Keyboard(Key::Left) => {
                    if index > 0 {
                        index -= 1;
                        visu.update_board(&args, board_array[index].clone());
                    }
                },
                _ => {},
            }
        }
    }
}