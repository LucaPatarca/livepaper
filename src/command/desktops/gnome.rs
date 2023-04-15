use super::Desktop;
use crate::{config::Config, command::utils::run_command};
use std::rc::Rc;

pub struct Gnome {
    commands: Vec<String>,
}

impl Gnome {
    pub fn new(config: Rc<Config>) -> Self {
        let commands = vec![
            format!(
                "gsettings set org.gnome.desktop.background picture-uri file://{}",
                config.save_path
            ),
            format!(
                "gsettings set org.gnome.desktop.background picture-uri-dark file://{}",
                config.save_path
            ),
        ];
        Self { commands }
    }
}

impl Desktop for Gnome {
    fn run(&self) -> Result<(), String> {
        Ok(for command in self.commands.iter() {
            run_command(command.to_owned())?
        })
    }
}
