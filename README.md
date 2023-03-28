# Hyprdock
A small utility to handle automatic monitor docking.

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

~~~
