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

    pub fn gen_wallpaper(&mut self, hour: u8, minute: u8) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut updated = false;
        for layer in self.layers.iter_mut() {
            updated = updated || layer.update(hour, minute);
        }
        if !updated {
            return None;
        }
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(self.config.frame_widht, self.config.frame_height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            for layer in self.layers.iter(){
                pixel.blend(&layer.get_pixel(x, y));
            }
        }
        Some(img)
    }
}
