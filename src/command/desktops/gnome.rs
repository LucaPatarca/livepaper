use super::Desktop;
use crate::config::Config;
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
    fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}
