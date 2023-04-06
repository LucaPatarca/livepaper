mod desktops;

use std::{process::Command, rc::Rc};

use crate::config::Config;

use self::desktops::{get_desktop, Desktop};

pub struct CommandRunner {
    engine: Box<dyn Desktop>,
}

impl CommandRunner {
    pub fn new(config: Rc<Config>) -> Self {
        Self { engine: get_desktop(Rc::clone(&config)) }
    }

    fn exec_command(&self, command: String) {
        let (exec, args) = if let Some((exec, args)) = command.split_once(" ") {
            (exec, args)
        } else {
            (command.as_str(), "")
        };
        Command::new(exec)
            .args(args.split(" "))
            .output()
            .expect("Cannot run command to change background");
    }

    pub fn change_wallpaper(&self) {
        for command in self.engine.get_commands() {
            self.exec_command(command);
        }
    }
}
