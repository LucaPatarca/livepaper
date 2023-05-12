# livepaper
 
 A rust program to dynamically update your wallpaper based on the time of day.
 
 ### Features
 - **compatible with linux and windows** (macos will probably come soon)
 - **automatically changes wallpaper**: it detects the current desktop and uses a set of commands to change the wallpaper automatically. 
Supported: Windows, Linux/Gnome, Linux/Hyprland (using hyprpaper) and more coming soon. In any other case you can set the ```desktop_command``` option.
 - **optimized regeneration**: It will not generate a new image if nothing changed from the last time
 - **BLAZINGLY FAST**

 ### Configuration
 
 livepaper can be configured by changing the defaults in its config file. The file is located in ```/home/$USER/.config/livepaper/config.toml``` for linux 
 and in ```%APPDATA%\livepaper\config\config.toml```.
 
 ##### Configurazion options:
 - **sunrise_start**: float, the time when the sunrise starts, default: 5.0
 - **sunrise_end**: float, the time when the sunrise ends, default: 9.0
 - **sunset_start**: float, the time when the sunset starts, default: 16.0
 - **sunset_end**: float, the time when the sunset ends, default: 20.0
 - **update_mins**: int, the number of minutes to wait between wallpaper updates, default: 10
 - **frame_height**: int, the wallpaper height in pixels, default: 1080
 - **frame_width**: int, the wallpaper width in pixels, default: 1920
 - **desktop_command**: Optional String, if needed it is possible to set a custom command to set the new wallpaper 
 - **save_path**: String, the path where the wallpaper result image is saved, must be a full path ending with a png file
 - **foreground_path**: Optional String, the path of the foreground png file, this must be a full path of a png file which 
has at least a trasnsparent section (otherwise you will not see the background color). This repo contains a default image 
(foreground.png), to use it, set this option to its path
 - **exec_loop**: boolean, whether livepaper should run continuously or not, normally you want to set it to true, default: false
