mod foreground;
mod gradient;
mod stars;

use self::{foreground::Foreground, gradient::Gradient, stars::Stars};
use crate::config::Config;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum LayerType {
    FOREGROUND,
    BACKGROUND,
    DECORATION,
}

pub trait Layer {
    fn get_type(&self) -> LayerType;
    fn update(&mut self, hour: u8, minute: u8) -> bool;
    fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8>;
}

pub fn get_layers(config: Rc<Config>) -> Vec<Box<dyn Layer>> {
    vec![
        Box::new(Gradient::new(Rc::clone(&config))),
        Box::new(Stars::new(Rc::clone(&config))),
        Box::new(Foreground::new(Rc::clone(&config))),
    ]
}
