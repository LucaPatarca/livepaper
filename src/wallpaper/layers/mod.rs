/*
 *
 * A layer is a trait and the Wallpaper struct contains a list of those
 *
 * Every layer has the functions:
 * update()
 * get_pixel(x,y)
 *
 * The layer list is loaded at startup based on configuration
 * 
 * On gen_wallpaper every layer is blended sequentially
 *
 *
*/

mod foreground;
mod gradient;
mod stars;

use std::rc::Rc;
use crate::config::Config;

use self::{gradient::Gradient, foreground::Foreground, stars::Stars};

pub trait Layer {
    fn update(&mut self, hour: u8, minute: u8);
    fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8>;
}

pub fn get_layers(config: Rc<Config>) -> Vec<Box<dyn Layer>> {
    vec![
        Box::new(Gradient::new(Rc::clone(&config))),
        Box::new(Stars::new(Rc::clone(&config))),
        Box::new(Foreground::new(Rc::clone(&config))),
    ]
}
