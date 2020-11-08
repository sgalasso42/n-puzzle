// extern crate piston_window;
// use piston_window::*;

// // use crate::graphics::text_measure::{measure};
// use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
// use piston_window::*;
// use glutin_window::GlutinWindow as Window;
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,  PressEvent};
// use piston::window::WindowSettings;
// use graphics::character::CharacterCache;
// use graphics::*;

// use graphics::types::FontSize;
// use graphics::{Context, Text};



// trait DrawText {
//     fn draw_text(
//         &mut self,
//         text: &str,
//         r: [f64; 4],
//         color: [f32; 4],
//         size: FontSize,
//         // halign: TextAlignment,
//         // valign: TextVerticalAlignment,
//         glyphs: &mut GlyphCache,
//         c: &Context,
//     );
// }

// impl DrawText for GlGraphics {
//     fn draw_text(
//         &mut self,
//         text: &str,
//         r: [f64; 4],
//         color: [f32; 4],
//         size: FontSize,
//         // halign: TextAlignment,
//         // valign: TextVerticalAlignment,
//         glyphs: &mut GlyphCache,
//         c: &Context,
//     ) {
//         let x0 = r[0];
//         let y0 = r[1];
//         let x1 = r[2];
//         let y1 = r[3];

//         let t = Text::new_color(color, size);
//         let size = t.measure(text, glyphs).unwrap();
//         fn center(p0: f64, p1: f64, wh: f64) -> f64 {
//             p0 + ((p1 - p0) / 2.0) - (wh / 2.0)
//         }
//         let x = 0.0;
//         // match halign {
//         //     TextAlignment::Left => x0,
//         //     TextAlignment::Right => x1 - size.width,
//         //     TextAlignment::Center => center(x0, x1, size.width),
//         // };

//         let y = 0.0;
//         // match valign {
//         //     TextVerticalAlignment::Top => y0,
//         //     TextVerticalAlignment::Bottom => y1 - size.height,
//         //     TextVerticalAlignment::Center => center(y0, y1, size.height),
//         // };

//         let transform = c.transform.trans(x, y);
//         let draw_state = c.draw_state;
//         t.draw(text, glyphs, &draw_state, transform, self).unwrap();
//     }
// }


// trait MeasureText {
//     fn measure<C>(
//         &self, 
//         text: &str, 
//         cache: &mut C) -> Result<Size, ()>
//     where
//         C: CharacterCache;
// }

// impl MeasureText for Text {
//     fn measure<C>(
//         &self, 
//         text: &str, 
//         cache: &mut C) -> Result<Size, ()>
//     where
//         C: CharacterCache,
//     {
//         let mut w = 0.0;
//         let mut h = 0.0;
//         for ch in text.chars() {
//             let character = cache.character(self.font_size, ch)
//                 .ok().unwrap();
//             let (left, top) = (character.left(), character.top());
//             w += character.advance_width() + left;
//             h = (character.advance_height() + top).max(h);
//         }
//         let result = Size {
//             width: w as f64,
//             height: h as f64,
//         };
//         Ok(result)
//     }
// }