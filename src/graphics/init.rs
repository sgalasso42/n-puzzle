extern crate glutin_window;
extern crate piston_window;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

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
}

impl App {
    fn render(&mut self, args: &RenderArgs, window: &PistonWindow) {
        // use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let grid = grid::Grid {
            cols: 3,
            rows: 3,
            units: (args.window_size[0]) / 3.0 - 10.0,
        };
        let line = Line::new(RED, 1.0);

        // let rotation = self.rotation;
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
                    // assert_eq!(iter.next(), Some((x, y)));
                    println!("Got: {:?}", (x, y));
                    println!("cell pos: {:?}", grid.cell_position((x, y)));
                    let pos = grid.cell_position((x, y));
                    let transform2 = c.transform.trans(pos[0], pos[1]);
                    text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                        "0",
                        &mut glyph_cache,
                        &c.draw_state,
                        transform2, gl
                    ).unwrap();
                    
                }
            }
                    println!("HEY");

            // Draw a box rotating around the middle of the screen.
            // grid(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
                    println!("YO");

        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

pub fn graphics() {
    // Change this to OpenGL::V2_1 if not working.
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
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &window);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }

        // if let Some(args) = e.press_args() {
        //     // app.key_press(args);
        // }
    }
}