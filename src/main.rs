mod command;
mod config;
mod consts;
mod wallpaper;

use chrono::{Local, Timelike};
use command::CommandRunner;
use std::{process::exit, rc::Rc, time::Duration};
use wallpaper::Wallpaper;
use clap::Parser;

//TODO get from cargo
const APP_NAME: &str = "circadian_wallpaper";
const CONFIG_NAME: &str = "config";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    hour: Option<u8>,

    #[arg()]
    minute: Option<u8>,

    #[arg(long)]
    gen_all: Option<String>,
}

fn main() {
    let args = Args::parse();
    let config: config::Config =
        confy::load(APP_NAME, Some(CONFIG_NAME)).expect("Cannot load config");
    if !config.is_valid() {
        println!(
            "Config file is not valid, edit the config file at: {} and come back.",
            confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
                .unwrap()
                .into_os_string()
                .to_string_lossy()
        );
        exit(1);
    }
    let time = Local::now();
    let mut hour = time.hour() as u8;
    let mut minute = time.minute() as u8;
    let config = Rc::new(config);
    let mut wallpaper = Wallpaper::new(Rc::clone(&config));
    let cmd_runner = CommandRunner::new(Rc::clone(&config));
    if let Some(path) = args.gen_all {
        for h in 0..24 {
            for m in (0..60).step_by(5){
                let img = wallpaper.gen_wallpaper(h, m);
                match img {
                    Some(img) => {
                        img.save(format!("{}/{:0>2}-{:0>2}.png", path, h, m)).unwrap();
                    }
                    None => {}
                }
            }
        }
    } else if config.exec_loop {
        loop {
            let img = wallpaper.gen_wallpaper(hour, minute);
            match img {
                Some(img) => {
                    img.save(&config.save_path).unwrap();
                    cmd_runner.change_wallpaper();
                }
                None => {}
            }
            std::thread::sleep(Duration::from_secs(config.update_mins * 60));
        }
    } else {
        if let Some(arg_hour) = args.hour {
            hour = arg_hour;
        }
        if let Some(arg_min) = args.minute {
            minute = arg_min;
        }
        let img = wallpaper.gen_wallpaper(hour, minute);
        match img {
            Some(img) => {
                img.save(&config.save_path).unwrap();
                cmd_runner.change_wallpaper();
            }
            None => {}
        }
    }
}
