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
    fn get_commands(&self) -> Vec<String> {
        vec![self.command.clone()]
    }
}
