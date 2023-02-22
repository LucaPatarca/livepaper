mod config;
mod consts;
mod gradient;
mod wallpaper;

use std::{
    process::{exit, Command},
    rc::Rc,
    time::Duration,
};

use chrono::{Local, Timelike};
use config::Config;
use wallpaper::Wallpaper;

const APP_NAME: &str = "circadian_wallpaper";
const CONFIG_NAME: &str = "config";

fn change_wallpaper(config: Rc<Config>) {
    let command = if let Some(de) = &config.desktop_env {
        de.get_command()
    } else {
        String::from(config.desktop_command.as_ref().expect("Invalid config"))
    };
    let (exec, args) = command
        .split_once(" ")
        .expect("Command should be a valid shell command");
    Command::new(exec)
        .args(args.split(" "))
        .output()
        .expect("Cannot run command to change background");
}

fn main_loop(wallpaper: &mut Wallpaper, config: &Rc<Config>) {
    let time = Local::now();
    let img = wallpaper.gen_wallpaper(time.hour() as u8, time.minute() as u8);
    img.save(&config.save_path).unwrap();
    change_wallpaper(Rc::clone(config));
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
    if config.exec_loop {
        loop {
            main_loop(&mut wallpaper, &config);
            std::thread::sleep(Duration::from_secs(config.update_mins * 60));
        }
    } else{
        main_loop(&mut wallpaper, &config);
    }
}
