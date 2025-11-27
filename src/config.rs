use serde_derive::{Deserialize, Serialize};
use std::env;

use crate::api;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sunrise_start: f32,
    pub sunrise_end: f32,
    pub sunset_start: f32,
    pub sunset_end: f32,

    pub update_mins: u64,
    pub frame_height: u32,
    pub frame_widht: u32,

    pub desktop_env: Option<String>,
    pub desktop_command: Option<String>,

    pub save_path: String,
    pub foreground_path: Option<String>,

    pub exec_loop: bool,
    pub use_api: bool,

    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}

impl Config {
    pub fn get_time_from_api(&mut self) {
        if !self.use_api {
            return
        }
        let data = api::get_api_data(self.latitude.unwrap_or(0.), self.longitude.unwrap_or(0.));
        match data {
            Ok(data) => {
                self.sunset_start = data.results.sunset.time;
                self.sunset_end = data.results.last_light.time;
                self.sunrise_start = data.results.first_light.time;
                self.sunrise_end = data.results.sunrise.time;
            },
            Err(err) => println!("{:?}", err)
        };
    }
}

impl Default for Config {
    fn default() -> Self {
        let save_path = if env::consts::OS == "windows" {
            format!(
                "{}\\wallpaper.png",
                env::var("USERPROFILE").unwrap_or("C:".to_string())
            )
        } else if let Ok(user_home) = env::var("HOME") {
            format!("{}/.local/share/backgrounds/wallpaper.png", user_home)
        } else {
            String::from("/tmp/wallpaper.png")
        };
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
            save_path,
            foreground_path: None,
            exec_loop: false,
            use_api: true,
            latitude: None,
            longitude: None,
        }
    }
}

impl Config {
    pub fn is_valid(&self) -> bool {
        true
    }

    #[cfg(unix)]
    pub fn get_desktop_env(&self) -> String {
        self.desktop_env
            .as_ref()
            .unwrap_or(
                &env::var("XDG_CURRENT_DESKTOP")
                    .unwrap_or_default()
                    .to_lowercase(),
            )
            .to_owned()
    }
}
