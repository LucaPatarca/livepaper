use self::{custom::Custom, gnome::Gnome, hyprland::Hyprland};
use crate::config::Config;
use std::rc::Rc;

mod custom;
mod gnome;
mod hyprland;

pub trait Desktop {
    fn get_commands(&self) -> Vec<String>;
}

pub fn get_desktop(config: Rc<Config>) -> Box<dyn Desktop> {
    if let Some(command) = &config.desktop_command {
        Box::new(Custom::new(command.to_string()))
    } else {
        let name = config
            .get_desktop_env();
        match name.as_str() {
            "hyprland" => Box::new(Hyprland::new(config)),
            "gnome" => Box::new(Gnome::new(config)),
            &_ => panic!("Invalid desktop environment set"),
        }
    }
}
