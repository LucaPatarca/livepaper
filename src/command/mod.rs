mod desktops;
pub mod utils;

use std::rc::Rc;

use crate::config::Config;

use self::desktops::{get_desktop, Desktop};

pub struct CommandRunner {
    engine: Box<dyn Desktop>,
}

impl CommandRunner {
    pub fn new(config: Rc<Config>) -> Self {
        Self { engine: get_desktop(Rc::clone(&config)) }
    }

    pub fn change_wallpaper(&self) {
        self.engine.run().expect("Cannot run command to change wallpaper");
    }
}
