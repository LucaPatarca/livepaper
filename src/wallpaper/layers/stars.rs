use super::Layer;
use crate::config::Config;
use image::{Pixel, Rgba};
use rand::Rng;
use std::rc::Rc;

struct Star {
    x: u32,
    y: u32,
    size: u8,
    brightness: u8,
}

impl Star {
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba<u8>> {
        let dx = x.abs_diff(self.x);
        let dy = y.abs_diff(self.y);
        let d = dx + dy;
        if d < self.size as u32 {
            let b = self.brightness as f32 * (1. - ((d as f32 / self.size as f32) * 0.7));
            Some(Rgba([0xFF, 0xFF, 0xFF, b as u8]))
        } else {
            None
        }
    }
}

pub struct Stars {
    stars: Vec<Star>,
    opacity: f32,
    config: Rc<Config>,
}

impl Stars {
    pub fn new(config: Rc<Config>) -> Self {
        let mut rng = rand::thread_rng();
        // TODO improve generation: more small and less big
        let stars = (0..150)
            .map(|_| Star {
                x: rng.gen_range(0..=config.frame_widht),
                y: rng.gen_range(0..=config.frame_widht),
                size: rng.gen_range(1..=4),
                brightness: rng.gen_range(100..=255),
            })
            .collect();
        Self {
            stars,
            opacity: 0.,
            config,
        }
    }
}

impl Layer for Stars {
    fn update(&mut self, hour: u8, minute: u8) -> bool {
        let time: f32 = (hour as f32) + ((minute as f32) / 60.);
        let srs = self.config.sunrise_start;
        let sre = self.config.sunrise_end;
        let sss = self.config.sunset_start;
        let sse = self.config.sunset_end;
        let new_opacity = if time > sse || time < srs {
            1.
        } else if time > sss {
            (time - sss) / (sse - sss)
        } else if time < sre {
            (sre - time) / (sre - srs)
        } else {
            0.
        };
        if new_opacity == self.opacity {
            false
        } else {
            self.opacity = new_opacity;
            true
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8> {
        let mut pixel = Rgba([0, 0, 0, 0]);
        for star in self.stars.iter() {
            if let Some(p) = star.get_pixel(x, y) {
                pixel.blend(&p);
            }
        }
        Rgba([
            pixel[0],
            pixel[1],
            pixel[2],
            (pixel[3] as f32 * self.opacity) as u8,
        ])
    }
}
