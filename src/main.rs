mod command;
mod config;
mod consts;
mod wallpaper;

use chrono::{Local, Timelike};
use command::CommandRunner;
use config::Config;
use std::{env, process::exit, rc::Rc, time::Duration};
use wallpaper::Wallpaper;

const APP_NAME: &str = "circadian_wallpaper";
const CONFIG_NAME: &str = "config";

fn main_loop(wallpaper: &mut Wallpaper, config: &Rc<Config>, cmd_runner: &CommandRunner) {
    let args: Vec<String> = env::args().collect();
    let hour;
    let minute;
    if args.len() < 3 {
        let time = Local::now();
        hour = time.hour() as u8;
        minute = time.minute() as u8;
    } else {
        hour = u8::from_str_radix(args[1].as_str(), 10)
            .expect(&format!("{} is not an integer", args[1]));
        minute = u8::from_str_radix(args[2].as_str(), 10)
            .expect(&format!("{} is not an integer", args[2]));
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

fn main() {
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
    let config = Rc::new(config);
    let mut wallpaper = Wallpaper::new(Rc::clone(&config));
    let cmd_runner = CommandRunner::new(Rc::clone(&config));
    if config.exec_loop {
        loop {
            main_loop(&mut wallpaper, &config, &cmd_runner);
            std::thread::sleep(Duration::from_secs(config.update_mins * 60));
        }
    } else {
        main_loop(&mut wallpaper, &config, &cmd_runner);
    }
}
