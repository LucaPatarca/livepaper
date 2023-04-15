use crate::command::utils::run_command;
use super::Desktop;

pub struct Custom {
    command: String,
}

impl Custom {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

impl Desktop for Custom {
    fn run(&self) -> Result<(), String> {
        run_command(self.command.clone())
    }
}
