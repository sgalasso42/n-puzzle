extern crate glutin_window;
extern crate piston_window;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;
use crate::board::utils::*;

// use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,  PressEvent};
// use piston::window::WindowSettings;
use piston_window::*;
use opengl_graphics::GlyphCache;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    board: Vec<i32>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let grid = grid::Grid {
            cols: 3,
            rows: 3,
            units: (args.window_size[0]) / 3.0 - 10.0,
        };
        let line = Line::new(RED, 1.0);

        let board = self.board.clone();
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        let ref font = assets.join("font.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
        // let factory = window.factory.clone();
        // let ts = TextureSettings::new();
        // let mut glyphs = Glyphs::new(font, ts, factory).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            
            // Clear the screen.
            clear(GREEN, gl);
            let transform = c
                .transform
                .trans(10.0 * 3.0 / 2.0, 10.0 * 3.0 / 2.0);
                // .rot_rad(rotation)
            grid.draw(&line, &c.draw_state, transform, gl);
            let mut iter = grid.cells();
            for y in 0..3 {
                for x in 0..3 {
                    // println!("Got: {:?}", (x, y));
                    // println!("cell pos: {:?}", grid.cell_position((x, y)));
                    let pos = grid.cell_position((x, y));
                    let transform2 = c.transform.trans(pos[0] + (grid.units / 2.0) + 10.0, pos[1] + (grid.units / 2.0) + 10.0);
                    let nb = board[fdtos(x as i32, y as i32, 3) as usize];
                    let mut string: String = "".to_string();
                    if nb == 3 * 3 {
                        string = "*".to_string();
                    } else {
                        string = nb.to_string(); 
                    }
                    text::Text::new_color([0.0, 0.5, 0.0, 1.0], 64).draw(
                        &string,
                        &mut glyph_cache,
                        &c.draw_state,
                        transform2, gl
                    ).unwrap();
                    
                }
            }
            // Draw a box rotating around the middle of the screen.
            // grid(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }

    fn updateBoard(&mut self, args: &Button, board: Vec<i32>) {
        println!("function update right");

        // Rotate 2 radians per second.
        self.board = board;
    }
}

pub fn graphics() {
    // Change this to OpenGL::V2_1 if not working.
    let all = [vec![3, 2, 6, 1, 4, 9, 8, 7, 5], vec![3, 2, 6, 1, 9, 4, 8, 7, 5], vec![3, 2, 6, 9, 1, 4, 8, 7, 5], vec![3, 2, 9, 6, 1, 4, 8, 7, 5]];
    let mut index: usize = 0;
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    // let mut window: PistonWindow = WindowSettings::new("npuzzle", [500, 500])
    //     .graphics_api(opengl)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();

    let mut window: PistonWindow = WindowSettings::new(
                "npuzzle",
                [500, 500]
            )
            .graphics_api(opengl)
            .fullscreen(false)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();
            // .fullscreen(true)
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        board: all[index].clone(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::Right) => {
                    println!("Right");
                    if index < all.len() - 1 {
                        index += 1;
                        app.updateBoard(&args, all[index].clone());
                    }
                },
                Button::Keyboard(Key::Left) => {
                    println!("Left");
                    if index > 0 {
                        index -= 1;
                        app.updateBoard(&args, all[index].clone());
                    }
                },
                _ => println!("Ain't special"),
            }
        }
    }
}