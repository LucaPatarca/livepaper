use super::Layer;
use crate::config::Config;
use crate::consts::*;
use colorgrad::Color;
use std::rc::Rc;

#[derive(PartialEq)]
enum State {
    UNKNOWN,
    NIGHT,
    SUNRISE,
    DAY,
    SUNSET,
}

pub struct Gradient {
    config: Rc<Config>,
    gradient: Option<colorgrad::Gradient>,
    last_state: State,
}

impl Gradient {
    pub fn new(config: Rc<Config>) -> Self {
        Gradient {
            config,
            gradient: None,
            last_state: State::UNKNOWN,
        }
    }

    fn blend_colors(color1: [u8; 3], color2: [u8; 3], blend: f32) -> [u8; 3] {
        let cr = color1[0] as f32;
        let cg = color1[1] as f32;
        let cb = color1[2] as f32;
        let nr = color2[0] as f32;
        let ng = color2[1] as f32;
        let nb = color2[2] as f32;
        let br = ((nr * blend) + (cr * (1. - blend))).round() as u8;
        let bg = ((ng * blend) + (cg * (1. - blend))).round() as u8;
        let bb = ((nb * blend) + (cb * (1. - blend))).round() as u8;
        return [br, bg, bb];
    }

    fn blend_gradients(
        time: f32,
        start: f32,
        end: f32,
        gradient_collection: [[[u8; 3]; 6]; 5],
        gradient_default: [[u8; 3]; 6],
    ) -> [[u8; 3]; 6] {
        let offset_time = time - start;
        let lenght = end - start;
        let index_f = offset_time / lenght * 5.;
        let index = index_f.floor() as usize;
        let blend = index_f - index as f32;
        let cur_colors = gradient_collection[index];
        let next_colors = if index > 3 {
            gradient_default
        } else {
            gradient_collection[index + 1]
        };
        let mut blended_colors = gradient_default;
        for i in 0..6 {
            let blend_color = Self::blend_colors(cur_colors[i], next_colors[i], blend);
            blended_colors[i] = blend_color;
        }
        return blended_colors;
    }
}

impl Layer for Gradient {
    fn update(&mut self, hour: u8, minute: u8) -> bool {
        let time = hour as f32 + (minute as f32 / 60.);
        let new_state = if time <= self.config.sunrise_start || time >= self.config.sunset_end {
            State::NIGHT
        } else if time >= self.config.sunrise_end && time <= self.config.sunset_start {
            State::DAY
        } else if time > self.config.sunrise_start && time < self.config.sunrise_end {
            State::SUNRISE
        } else if time > self.config.sunset_start && time < self.config.sunset_end {
            State::SUNSET
        } else {
            State::UNKNOWN
        };
        if new_state == self.last_state && (new_state == State::DAY || new_state == State::NIGHT) {
            return false;
        }
        let colors_rgb = match new_state {
            State::NIGHT => NIGHT_COLORS,
            State::DAY => DAY_COLORS,
            State::SUNRISE => Self::blend_gradients(
                time,
                self.config.sunrise_start,
                self.config.sunrise_end,
                SUNRISE_COLORS,
                DAY_COLORS,
            ),
            State::SUNSET => Self::blend_gradients(
                time,
                self.config.sunset_start,
                self.config.sunset_end,
                SUNSET_COLORS,
                NIGHT_COLORS,
            ),
            State::UNKNOWN => BLACK_GRADIENT,
        };
        let mut colors: Vec<Color> = Vec::new();
        for c in colors_rgb {
            colors.push(Color::from_rgba8(c[0], c[1], c[2], 255));
        }
        self.gradient = Some(
            colorgrad::CustomGradient::new()
                .colors(&colors)
                .domain(&DOMAINS)
                .build()
                .unwrap(),
        );
        true
    }

    fn get_pixel(&self, _x: u32, y: u32) -> image::Rgba<u8> {
        if let Some(grad) = &self.gradient {
            let grad_pos = (y as f64) / (self.config.frame_height as f64);
            let grad_rgba = grad.at(grad_pos).to_rgba8();
            image::Rgba(grad_rgba)
        } else {
            image::Rgba([0, 0, 0, 0])
        }
    }
}
