use super::{Desktop, utils::run_command};
use crate::config::Config;
use serde_derive::Deserialize;
use std::{process::Command, rc::Rc};

#[derive(Deserialize)]
struct Monitor {
    name: String,
}

pub struct Hyprland {
    commands: Vec<String>,
    config: Rc<Config>,
}

impl Hyprland {
    pub fn new(config: Rc<Config>) -> Self {
        let commands = vec![
            format!("hyprctl hyprpaper unload {}", config.save_path),
            format!("hyprctl hyprpaper preload {}", config.save_path),
        ];
        Self { commands, config }
    }

    fn get_commands(&self) -> Vec<String> {
        let output = Command::new("hyprctl")
            .args(["monitors", "-j"])
            .output()
            .expect("Cannot get monitors");
        let monitors: Vec<Monitor> = serde_json::from_str(
            String::from_utf8(output.stdout)
                .expect("Cannot get monitors")
                .as_str(),
        )
        .expect("Cannot get monitors");
        let mut commands = self.commands.clone();
        for monitor in monitors {
            commands.push(format!(
                "hyprctl hyprpaper wallpaper {},{}",
                monitor.name, self.config.save_path
            ));
        }
        commands
    }
}

impl Desktop for Hyprland {
    fn run(&self) -> Result<(), String> {
        Ok(for command in self.get_commands(){
            run_command(command)?
        })
    }
}
