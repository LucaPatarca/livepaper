use std::process::Command;

use chrono::{Local, Timelike};
use colorgrad::Color;
use image::{io::Reader, Pixel};

const TIMES_RISE: [f64;7] = [-0.2, 0.1, 0.3, 0.4, 0.5, 0.55, 1.5]; 
const TIMES_SET: [f64;7] = [1.5, 0.55, 0.5, 0.4, 0.3, 0.1, -0.2]; 
const HEIGHT_FRAME: f32 = 1080.; 
const WIDTH_FRAME: f32 = 1920.;
// const UPDATE_MINS: i32 = 5; 
// const ENABLE_SUN: bool = true; 
// const SUN_W: i32 = 128; 
// const SUN_H: i32 = 128; 

fn gen_image(hour: u32, minute: u32){
    let civt_begin = 7.0;
    let civt_end = 17.5;
    let colors = [
      Color::from_rgba8(36,39,50,255), 
      Color::from_rgba8(38,81,140,255), 
      Color::from_rgba8(91,140,132,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(125,196,228,255), 
      Color::from_rgba8(125,196,228,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(238,212,159,255), 
      Color::from_rgba8(116,179,169,255), 
      Color::from_rgba8(38,81,140,255), 
      Color::from_rgba8(36,39,50,255), 
    ];
    let offset = (((minute as f32 / 60.) + (hour as f32)) - 1.) * HEIGHT_FRAME;

    let mut domain: Vec<f64> = Vec::new();

    let colors_len = colors.len();
    for i in 0..colors_len{
        if i < colors_len/2 {
            domain.push((TIMES_RISE[i] + civt_begin) / 24.);
        } else{
            domain.push((civt_end - TIMES_SET[i - colors_len/2]) / 24.);
        }
    }

    let grad = colorgrad::CustomGradient::new()
    .colors(&colors)
    .domain(&domain[..])
    .build().unwrap();

    let mut img = Reader::open("./foreground.png").unwrap().decode().unwrap()
        .resize_exact(WIDTH_FRAME as u32, HEIGHT_FRAME as u32, image::imageops::FilterType::Nearest).to_rgba8();

    for (_,y,pixel) in img.enumerate_pixels_mut(){
        let grad_pos = ((y as f64) + (offset as f64)) / ((HEIGHT_FRAME as f64) * 24.);
        let grad_rgba = grad.at(grad_pos).to_rgba8();
        let mut grad_pixel = image::Rgba(grad_rgba);
        grad_pixel.blend(pixel);
        *pixel = grad_pixel;
    }

    img.save("/tmp/background.png").unwrap();
}

fn main() {
    let time = Local::now();
    gen_image(time.hour(), time.minute());
    Command::new("swaymsg")
        .arg("output")
        .arg("\"*\"")
        .arg("bg")
        .arg("/tmp/background.png")
        .arg("fill")
        .output()
        .expect("Cannot run command to change background");

}
