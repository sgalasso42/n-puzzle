extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;

use crate::board::utils::*;

use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston_window::*;
use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::{Context, Text};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextAlignment {
    Left, Right, Center
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextVerticalAlignment {
    Top, Bottom, Center
}

pub struct Visu {
    gl: GlGraphics,
    board: Vec<i32>,
    size: i32,
    time: String
}

impl Visu {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let size = self.size;

        let grid = grid::Grid {
            cols: size as u32,
            rows: size as u32,
            units: (args.window_size[0]) / size as f64 - (20.0 / size as f64),
        };

        let line = Line::new(RED, 1.5);
        
        // let (win_w, win_h) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let board = self.board.clone();
        let time = self.time.clone();
        
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        let ref font = assets.join("font.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            
            let transform = c
                .trans(10.0, 10.0);
            
            grid.draw(&line, &c.draw_state, transform, gl);
            // let iter = grid.cells();
            for y in 0..size as u32 {
                for x in 0..size as u32 {
                    let pos = grid.cell_position((x, y));
                    let nb = board[fdtos(x as i32, y as i32, size) as usize];
                    let string: String;
                    
                    if nb == size * size {
                        string = "*".to_string();
                    } else {
                        string = nb.to_string(); 
                    }
                    let r = [pos[0] + 10.0, pos[1] + 10.0, pos[0] + 10.0 + grid.units, pos[1] + 10.0 + grid.units];
                    gl.draw_text(&string, r, RED, ((64.0 * (5.0 / size as f32)) as u32) as u32, TextAlignment::Center, TextVerticalAlignment::Center, &mut glyph_cache, &c);
                }
            }
            let duration: &String = &("Duration : ".to_string() + &time + &("s".to_string()));
            let r = [10.0, 510.0, 490.0, 540.0];
            gl.draw_text(&duration, r, RED, 32, TextAlignment::Center, TextVerticalAlignment::Center, &mut glyph_cache, &c);
        });
    }

    // fn update(&mut self, args: &UpdateArgs) {
    // }

    fn update_board(&mut self, _args: &Button, board: Vec<i32>) {
        self.board = board;
    }
}

pub fn visualisator(board_array: &[Vec<i32>], size: i32, time: String) {
    
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


trait DrawText {
    fn draw_text(
        &mut self,
        text: &str,
        r: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: TextAlignment,
        valign: TextVerticalAlignment,
        glyphs: &mut GlyphCache,
        c: &Context,
    );
}

impl DrawText for GlGraphics {
    fn draw_text(
        &mut self,
        text: &str,
        r: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: TextAlignment,
        valign: TextVerticalAlignment,
        glyphs: &mut GlyphCache,
        c: &Context,
    ) {
        let x0 = r[0];
        let y0 = r[1];
        let x1 = r[2];
        let y1 = r[3];

        let t = Text::new_color(color, size);
        let size = t.measure(text, glyphs).unwrap();
        fn center_w(p0: f64, p1: f64, w: f64) -> f64 {
            p0 + ((p1 - p0) / 2.0) - (w / 2.0)
        }
        fn center_h(p0: f64, p1: f64, h: f64) -> f64 {
            p0 + ((p1 - p0) / 2.0) + (h / 2.0)
        }

        let x = match halign {
            TextAlignment::Left => x0,
            TextAlignment::Right => x1 - size.width,
            TextAlignment::Center => center_w(x0, x1, size.width),
        };

        let y = match valign {
            TextVerticalAlignment::Top => y0,
            TextVerticalAlignment::Bottom => y1 - size.height,
            TextVerticalAlignment::Center => center_h(y0, y1, size.height),
        };

        let transform = c.transform.trans(x, y);
        let draw_state = c.draw_state;
        t.draw(text, glyphs, &draw_state, transform, self).unwrap();
    }
}


trait MeasureText {
    fn measure<C>(
        &self, 
        text: &str, 
        cache: &mut C) -> Result<Size, ()>
    where
        C: CharacterCache;
}

impl MeasureText for Text {
    fn measure<C>(
        &self, 
        text: &str, 
        cache: &mut C) -> Result<Size, ()>
    where
        C: CharacterCache,
    {
        let mut w = 0.0;
        let mut h = 0.0;
        for ch in text.chars() {
            let character = cache.character(self.font_size, ch)
                .ok().unwrap();
            let (left, top) = (character.left(), character.top());
            w += character.advance_width() + left;
            h = (character.advance_height() + top).max(h);
        }
        let result = Size {
            width: w as f64,
            height: h as f64,
        };
        Ok(result)
    }
}