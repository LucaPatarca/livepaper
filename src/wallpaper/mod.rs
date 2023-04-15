use self::layers::{get_layers, LayerType};
use crate::config::Config;
use image::{ImageBuffer, Pixel, Rgba};
use std::rc::Rc;

mod layers;
use layers::Layer;

pub struct Wallpaper {
    config: Rc<Config>,
    layers: Vec<Box<dyn Layer>>,
}

impl Wallpaper {
    pub fn new(config: Rc<Config>) -> Self {
        Wallpaper {
            layers: get_layers(Rc::clone(&config)),
            config,
        }
    }

    pub fn gen_wallpaper(
        &mut self,
        hour: u8,
        minute: u8,
    ) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut updated = false;
        for layer in self.layers.iter_mut() {
            updated = layer.update(hour, minute) || updated;
        }
        if !updated {
            return None;
        }
        let mut buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.config.frame_widht, self.config.frame_height);
        let foregrounds: Vec<&Box<dyn Layer>> = self
            .layers
            .iter()
            .filter(|e| e.get_type() == LayerType::FOREGROUND)
            .collect();
        let backgrounds: Vec<&Box<dyn Layer>> = self
            .layers
            .iter()
            .filter(|e| e.get_type() == LayerType::BACKGROUND)
            .collect();
        let decorations: Vec<&Box<dyn Layer>> = self
            .layers
            .iter()
            .filter(|e| e.get_type() == LayerType::DECORATION)
            .collect();
        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            for layer in foregrounds.iter() {
                pixel.blend(&layer.get_pixel(x, y));
            }
            for layer in decorations.iter() {
                if pixel[3] == 0 {
                    pixel.blend(&layer.get_pixel(x, y));
                }
            }
            for layer in backgrounds.iter() {
                let mut pix = layer.get_pixel(x, y);
                pix.blend(&pixel);
                *pixel = pix;
            }
        }
        Some(buf)
    }
}
