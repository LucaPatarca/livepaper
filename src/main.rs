use std::{process::Command, time::Duration};

use chrono::{Local, Timelike};
use colorgrad::Color;
use image::{io::Reader, Pixel};

const HEIGHT_FRAME: f32 = 1080.; 
const WIDTH_FRAME: f32 = 1920.;
const UPDATE_MINS: u64 = 10; 

const SUNRISE_START:f32 = 5.;
const SUNRISE_END:f32 = 9.;
const SUNSET_START:f32 = 16.;
const SUNSET_END:f32 = 20.;

const NIGHT_COLORS:[[u8;3];6] = [
    [4, 9, 17],
    [7, 14, 27],
    [11, 24, 43],
    [19, 37, 60],
    [28, 50, 73],
    [36, 58, 76],
];

const DAY_COLORS:[[u8;3];6] = [
    [80, 167, 253],
    [92, 178, 254],
    [107, 189, 254],
    [121, 200, 254],
    [133, 209, 254],
    [141, 214, 254],
];
    
const SUNRISE_COLORS:[[[u8;3];6];5] = [
  NIGHT_COLORS,
  [
    [1, 1, 4],
    [10, 10, 18],
    [27, 26, 30],
    [48, 40, 34],
    [78, 44, 14],
    [94, 24, 1],
  ],
  [
    [105, 83, 113],
    [155, 99, 104],
    [188, 113, 97],
    [171, 95, 83],
    [148, 77, 71],
    [138, 69, 65],
  ],
  [
    [137,137,137],
    [163,148,139],
    [153,124,113],
    [123,93,91],
    [90,70,78],
    [64,55,67],
  ],
  DAY_COLORS,
];

const SUNSET_COLORS:[[[u8;3];6];5] = [
  DAY_COLORS,
  [
    [43,72,80],
    [75,111,126],
    [112,157,164],
    [174,189,154],
    [200,144,77],
    [74,58,48],
  ],
  [
    [1, 34, 84],
    [2, 56, 123],
    [30, 89, 166],
    [99, 121, 187],
    [123, 83, 127],
    [37, 54, 104],
  ],
  [
    [28, 58, 98],
    [30, 62, 104],
    [33, 68, 112],
    [36, 74, 123],
    [40, 81, 132],
    [43, 85, 139],
  ],
  NIGHT_COLORS,
];

const DOMAINS: [f64;6] = [
    0.0,
    0.31,
    0.56,
    0.75,
    0.89,
    1.,
];

fn blend_colors(color1:[u8;3], color2:[u8;3], blend: f32) -> [u8;3]{
  let cr = color1[0] as f32;
  let cg = color1[1] as f32;
  let cb = color1[2] as f32;
  let nr = color2[0] as f32;
  let ng = color2[1] as f32;
  let nb = color2[2] as f32;
  let br = ((nr*blend) + (cr*(1.-blend))).round() as u8;
  let bg = ((ng*blend) + (cg*(1.-blend))).round() as u8;
  let bb = ((nb*blend) + (cb*(1.-blend))).round() as u8;
  return [br,bg,bb];
}

fn get_colors(hour: u8, minute: u8) -> [[u8;3];6]{
  let time = hour as f32 + (minute as f32 / 60.);
  let mut cur_colors = DAY_COLORS;
  let mut next_colors = DAY_COLORS;
  let mut blend = 0.;
  if time <= SUNRISE_START || time >= SUNSET_END {
    return NIGHT_COLORS;
  } else if time >= SUNRISE_END && time <= SUNSET_START {
    return DAY_COLORS;
  } else if time > SUNRISE_START && time < SUNRISE_END {
    let offset_time = time-SUNRISE_START;
    let lenght = SUNRISE_END - SUNRISE_START;
    let index_f = offset_time/lenght*5.;
    let index = index_f.floor() as usize;
    blend = index_f - index as f32;
    cur_colors = SUNRISE_COLORS[index];
    if index > 3 {
        next_colors = DAY_COLORS;
    } else{
        next_colors = SUNRISE_COLORS[index+1];
    }
  } else if time > SUNSET_START && time < SUNSET_END {
    let offset_time = time-SUNSET_START;
    let lenght = SUNSET_END - SUNSET_START;
    let index_f = offset_time/lenght*5.;
    let index = index_f.floor() as usize;
    blend = index_f - index as f32;
    cur_colors = SUNSET_COLORS[index];
    if index > 3 {
        next_colors = NIGHT_COLORS;
    } else{
        next_colors = SUNSET_COLORS[index+1];
    }
  }
  let mut blended_colors = DAY_COLORS;
  for i in 0..6 {
    let blend_color = blend_colors(cur_colors[i], next_colors[i], blend);
    blended_colors[i] = blend_color;
  }
  return blended_colors;
}

fn gen_image(hour: u8, minute: u8){

    let colors_rgb = get_colors(hour, minute);
    let mut colors: Vec<Color> = Vec::new();
    for c in colors_rgb {
        colors.push(Color::from_rgba8(c[0], c[1], c[2], 255));
    }

    let grad = colorgrad::CustomGradient::new()
    .colors(&colors)
    .domain(&DOMAINS)
    .build().unwrap();

    let mut img = Reader::open("/home/luca/Pictures/foreground.png").unwrap().decode().unwrap()
        .resize_exact(WIDTH_FRAME as u32, HEIGHT_FRAME as u32, image::imageops::FilterType::Nearest).to_rgba8();

    for (_,y,pixel) in img.enumerate_pixels_mut(){
        let grad_pos = (y as f64) / (HEIGHT_FRAME as f64);
        let grad_rgba = grad.at(grad_pos).to_rgba8();
        let mut grad_pixel = image::Rgba(grad_rgba);
        grad_pixel.blend(pixel);
        *pixel = grad_pixel;
    }

    img.save("test.png").unwrap();
}

fn main() {
    loop {
        let time = Local::now();
        gen_image(time.hour() as u8, time.minute() as u8);
        let command = "swaymsg output \"*\" bg /tmp/background.png fill";
        let (exec, args) = command.split_once(" ").expect("Command should be a valid shell command");
        Command::new(exec)
            .args(args.split(" "))
            .output()
            .expect("Cannot run command to change background");
        std::thread::sleep(Duration::from_secs(UPDATE_MINS * 60));
    }
}
