use self::custom::Custom;
use crate::config::Config;
use std::rc::Rc;

mod custom;
#[cfg(unix)] mod gnome;
#[cfg(unix)] mod hyprland;
#[cfg(windows)] mod windows;

pub trait Desktop {
    fn run(&self) -> Result<(), String>;
}

#[cfg(windows)]
pub fn get_desktop(config: Rc<Config>) -> Box<dyn Desktop> {
    use self::windows::Windows;

    if let Some(command) = &config.desktop_command {
        Box::new(Custom::new(command.to_string()))
    } else {
        Box::new(Windows::new(config))
    }
}

#[cfg(unix)]
pub fn get_desktop(config: Rc<Config>) -> Box<dyn Desktop> {
    use self::{hyprland::Hyprland, gnome::Gnome};

    if let Some(command) = &config.desktop_command {
        Box::new(Custom::new(command.to_string()))
    } else {
        let name = config
            .get_desktop_env();
        match name.as_str() {
            "hyprland" => Box::new(Hyprland::new(config)),
            "gnome" | "ubuntu:gnome" => Box::new(Gnome::new(config)),
            &_ => panic!("Invalid desktop environment set"),
        }
    }
}
