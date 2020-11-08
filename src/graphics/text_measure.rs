// extern crate piston_window;
// use piston_window::*;

// use graphics::character::CharacterCache;
// use graphics::Text;
// use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
// use piston_window::*;
// use glutin_window::GlutinWindow as Window;
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,  PressEvent};
// use piston::window::WindowSettings;
// use graphics::*;

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