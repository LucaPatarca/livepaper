use std::rc::Rc;
use crate::config::Config;
use super::Desktop;

extern crate winapi;
use winapi::ctypes::c_void;
use winapi::um::winuser::{SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};
use std::ffi::CString;

pub struct Windows {
    path: String
}

impl Windows {
    pub fn new(config: Rc<Config>) -> Self {
        Self {path: config.save_path.clone()}
    }
}

impl Desktop for Windows {
    fn run(&self) -> Result<(), String> {
        let image_path = CString::new(self.path.as_str()).unwrap();
        Ok(unsafe {
            SystemParametersInfoA(
                SPI_SETDESKWALLPAPER,
                0,
                image_path.as_ptr() as *mut c_void,
                SPIF_UPDATEINIFILE,
            );
        })
    }
}
