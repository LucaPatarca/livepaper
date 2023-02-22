use std::rc::Rc;

use crate::config::Config;
use crate::gradient::Gradient;

use image::{io::Reader, ImageBuffer, Pixel, Rgba};

pub struct Wallpaper {
    foreground: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    gradient: Gradient,
    config: Rc<Config>,
}

impl Wallpaper {
    fn load_foreground(path: &str, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        Reader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .resize_exact(width, height, image::imageops::FilterType::Nearest)
            .to_rgba8()
    }

    pub fn new(config: Rc<Config>) -> Self {
        let mut foreground = None;
        if let Some(path) = &config.foreground_path{
            foreground = Some(Self::load_foreground(path, config.frame_widht, config.frame_height));
        };
        Wallpaper {
            foreground,
            gradient: Gradient::new(Rc::clone(&config)),
            config,
        }
    }

    pub fn gen_wallpaper(&mut self, hour: u8, minute: u8) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.gradient.update(hour, minute);
        let mut img = ImageBuffer::new(self.config.frame_widht, self.config.frame_height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut grad_pixel = self.gradient.get_pixel(y);
            if let Some(foreground) = &self.foreground{
                let fg_pixel = foreground.get_pixel(x,y);
                grad_pixel.blend(fg_pixel);
            }
            *pixel = grad_pixel;
        }
        img
    }
}
