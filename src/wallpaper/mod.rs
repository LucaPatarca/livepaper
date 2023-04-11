use crate::config::Config;
use image::{ImageBuffer, Pixel, Rgba};
use std::rc::Rc;

mod layers;
use layers::Layer;

use self::layers::get_layers;

pub struct Wallpaper {
    config: Rc<Config>,
    layers: Vec<Box<dyn Layer>>
}

impl Wallpaper {
    pub fn new(config: Rc<Config>) -> Self {
        Wallpaper {
            layers: get_layers(Rc::clone(&config)),
            config,
        }
    }

    pub fn gen_wallpaper(&mut self, hour: u8, minute: u8) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        for layer in self.layers.iter_mut() {
            layer.update(hour, minute);
        }
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(self.config.frame_widht, self.config.frame_height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            for layer in self.layers.iter(){
                pixel.blend(&layer.get_pixel(x, y));
            }
            // let mut grad_pixel = if stars.contains(&(x,y)) {
                // let b = rng.gen_range(100..255);
                // Rgba::from([b, b, b, 0xFF])
            // } else {
                // self.gradient.get_pixel(y)
            // };
            // if let Some(foreground) = &self.foreground {
                // let fg_pixel = foreground.get_pixel(x, y);
                // grad_pixel.blend(fg_pixel);
            // }
            // *pixel = grad_pixel;
        }
        img
    }
}
