# Hyprdock
A small utility to handle automatic monitor docking.

## Breaking Changes
- change of config directory from hyprland config directory to hyprdock directory

## HardRequirements
- acpid installed and running

### Soft Requirements
These are programs that were meant to be used with hyprdock, however, the toml allows you to specify the commands you would like to run.
So feel free to test out other programs.
- Hyprland
- swaylock
- systemd

## behavior and features
- server mode 
  - if there is a monitor connected and you close the laptop lid, the laptop will be stopped from suspending and instead uses the external monitor
  - without a monitor connected, closing the lid will suspend the laptop
  - the name for the monitor is the first monitor specified in the hyprland.conf
- external: switches to external monitor only
- internal: switches to internal monitor only
- extend: extends monitors
- mirror: mirrors monitors
- export: save your current monitor configuration\
          optional name parameter -> save configuration with specific name
- import: import a stored configuration\
          optional name parameter -> load configuration with specific name

## Example config
### path needs to be $HOME/.config/hypr/hyprdock.toml
~~~
monitor_name = "eDP-1"
open_bar_command = "eww open bar"
close_bar_command = "eww close-all"
reload_bar_command = "eww reload"
suspend_command = "systemctl suspend"
lock_command = "swaylock -c 000000"
utility_command = "playerctl --all-players -a pause"
get_monitors_command = "hyprctl monitors"
enable_internal_monitor_command = "hyprctl keyword monitor eDP-1,highrr,0x0,1"
disable_internal_monitor_command = "hyprctl keyword monitor eDP-1,disabled"
enable_external_monitor_command = "hyprctl keyword monitor ,highrr,0x0,1"
disable_external_monitor_command = "hyprctl keyword monitor ,disabled"
extend_command = "hyprctl keyword monitor ,highrr,1920x0,1"
mirror_command = "hyprctl keyword monitor ,highrr,0x0,1"
wallpaper_command = "hyprctl dispatch hyprpaper"
css_string = ""
~~~

### multiple commands in 1 line
~~~
# make sure to leave a blank space between the start and end of the ;;, which marks the end and start of commands.
utility_command = "yourcommand args ;; yourothercommand args"
~~~

### extend and mirror
You can specify which should be the default command after plugging in monitors or opening the laptop lid when an external monitor is still connected:
~~~
default_external_command = "extend" # or "mirror"
~~~

### When are specific functions called?
- open_bar_command =>  used to open new bars on new monitors
- close_bar_command =>  used to close a bugged eww bar
- reload_bar_command =>  used to remove graphical errors with eww after re-enabling internal monitor
- suspend_command =>  used to suspend (on lid close without external monitor)
- lock_command => used to lock screen (on lid close without external monitor)
- utility_command => used before locking -> stop music etc
- get_monitors_command => used to check if external monitors are attached
- enable_internal_monitor_command => run after using internal only or opening the laptop lid after using external monitor only
- disable_internal_monitor_command => run after using external only or closing laptop lid with external monitor attached
- enable_external_monitor_command => run after disabling internal monitor or pluggin in an external monitor
- disable_external_monitor_command => run after using internal only or unplugging an external monitor
- extend_command => run after using extend or default function for external monitors
- mirror_command => run after using mirror or default function for external monitors
- wallpaper_command => run after plugging in a monitor

### CSS
By default hyprdock uses your system gtk4 theme.\
The style can be configured with your own CSS file.\
Just overwrite the css_string variable in the toml configuration file.

List of IDs 
- MainWindow 
- MainBox
- InternalButton
- ExternalButton
- ExtendButton
- MirrorButton
- ExportButton
