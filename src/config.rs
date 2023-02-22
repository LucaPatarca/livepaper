use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DE {
    KDE,
    GNOME,
    XFCE,
}

impl DE {
    pub fn get_command(&self) -> String {
        match self {
            DE::KDE => todo!(),
            DE::GNOME => todo!(),
            DE::XFCE => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sunrise_start: f32,
    pub sunrise_end: f32,
    pub sunset_start: f32,
    pub sunset_end: f32,

    pub update_mins: u64,
    pub frame_height: u32,
    pub frame_widht: u32,

    pub desktop_env: Option<DE>,
    pub desktop_command: Option<String>,

    pub save_path: String,
    pub foreground_path: Option<String>,

    pub exec_loop: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sunrise_start: 5.,
            sunrise_end: 9.,
            sunset_start: 16.,
            sunset_end: 20.,
            update_mins: 10,
            frame_height: 1080,
            frame_widht: 1920,
            desktop_env: None,
            desktop_command: None,
            save_path: String::from("/tmp/wallpaper.png"),
            foreground_path: None,
            exec_loop: false,
        }
    }
}

impl Config {
    pub fn is_valid(&self) -> bool {
        self.desktop_env.is_some() ||
        self.desktop_command.is_some()
    }
}
