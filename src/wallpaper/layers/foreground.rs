use super::Layer;
use crate::config::Config;
use image::{io::Reader, ImageBuffer, Rgba};
use std::rc::Rc;

pub struct Foreground {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Foreground {
    pub fn new(config: Rc<Config>) -> Self {
        let buffer = Reader::open(
            config
                .foreground_path
                .as_ref()
                .expect("Cannot load foreground"),
        )
        .expect("Cannot load foreground")
        .decode()
        .unwrap()
        .resize_exact(
            config.frame_widht,
            config.frame_height,
            image::imageops::FilterType::Nearest,
        )
        .to_rgba8();
        Self { buffer }
    }
}

impl Layer for Foreground {
    fn update(&mut self, _hour: u8, _minute: u8) -> bool {
        false
    }

    fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8> {
        self.buffer.get_pixel(x, y).to_owned()
    }
}
