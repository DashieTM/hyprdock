# Hyprdock
A small utility to handle automatic monitor docking.

## Requirements
- Hyprland installed... duh
- systemd for suspend -> functionality for parsing custom commands will follow
- swaylock for locking the screen -> see above
- acpid installed and running

## behavior and features
- server mode 
  - if there is a monitor connected and you close the laptop lid, the laptop will be stopped from suspending and instead uses the external monitor
  - without a monitor connected, closing the lid will suspend the laptop
  - the name for the monitor is the first monitor specified in the hyprland.conf
- external: switches to external monitor only
- internal: switches to internal monitor only
- extend: extends monitors
- mirror: mirrors monitors
