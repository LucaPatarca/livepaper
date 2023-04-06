use std::rc::Rc;
use crate::config::Config;
use super::Desktop;

const PS_CLASS: &'static str = "using System.Runtime.InteropServices;\
public class Wallpaper\
{\
  public const int SetDesktopWallpaper = 20;\
  public const int UpdateIniFile = 0x01;\
  public const int SendWinIniChange = 0x02;\
  [DllImport(\"user32.dll\", SetLastError = true, CharSet = CharSet.Auto)]\
  private static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);\
  public static void SetWallpaper(string path)\
  {\
    SystemParametersInfo(SetDesktopWallpaper, 0, path, UpdateIniFile | SendWinIniChange);\
  }\
}";

pub struct Windows {
    commands: Vec<String>
}

impl Windows {
    pub fn new(config: Rc<Config>) -> Self {
        Self {commands: vec![
            format!("Add-Type '{}'", PS_CLASS),
            format!("[Wallpaper]::SetWallpaper(\"{}\")", config.save_path)
        ]}
    }
}

impl Desktop for Windows {
    fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}
