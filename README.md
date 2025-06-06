# Hyprdock

A small utility to handle automatic monitor docking for Hyprland.

## Hard Requirements for "server" mode

- Acpid installed and running
- Lid events ignored by logind/other

## Behavior and features

- server mode
  - if there is a monitor connected and you close the laptop lid, the laptop will be stopped from suspending and instead uses the external monitor
  - without a monitor connected, closing the lid will suspend/hibernate the laptop
  - the name for the monitor is the first monitor specified in the hyprland.conf
- external: switches to external monitor only
- internal: switches to internal monitor only
- extend: extends monitors
- mirror: mirrors monitors
- export: save your current monitor configuration\
   optional name parameter -> save configuration with specific name
- import: import a stored configuration\
   optional name parameter -> load configuration with specific name

# Configuration

## Example toml file

Location: `$XDG_CONFIG_HOME/hyprdock/config.toml`

```toml
default_external_mode = "extend"
css_string = ""

[open_bar_command]
base = "ironbar"
args = []

[close_bar_command]
base = "killall"
args = ["ironbar"]

[reload_bar_command]
base = "ironbar"
args = []

[suspend_command]
base = "systemctl"
args = ["suspend"]

[lock_command]
base = "hyprlock"
args = []

[utility_command]
base = "playerctl"
args = ["--all-players", "-a", "pause"]

[get_monitors_command]
base = "hyprctl"
args = ["monitors"]

[enable_internal_monitor_command]
base = "hyprctl"
args = ["keyword", "monitor", "eDP-1,highres,0x0,1"]

[disable_internal_monitor_command]
base = "hyprctl"
args = ["keyword", "monitor", "eDP-1,disabled"]

[enable_external_monitor_command]
base = "hyprctl"
args = ["keyword", "monitor", ",highres,0x0,1"]

[disable_external_monitor_command]
base = "hyprctl"
args = ["keyword", "monitor", ",disabled"]

[extend_command]
base = "hyprctl"
args = ["keyword", "monitor", ",highres,1920x0,1"]

[mirror_command]
base = "hyprctl"
args = ["keyword", "monitor", ",highres,0x0,1"]

[wallpaper_command]
base = "hyprctl"
args = ["dispatch", "hyprpaper"]
```

### Lid switch

In order to use the server mode properly, you need to override existing lid switch behavior.
For systemd/logind, you can ignore the lid switch events like this in the file `/etc/systemd/logind.conf`:

```conf
HandleLidSwitch=ignore
HandleLidSwitchExternalPower=ignore
HandleLidSwitchDocked=ignore
```

### Nix/Home-manager

As of right now, hyprdock is not packaged in upstream NixOS or home-manager.
The NixOS PR is open, with a home-manager PR following after.
You therefore have to include hyprdock as a standalone flake:

```nix
inuts = {
    hyprdock.url = "github:Xetibo/hyprdock";
};
```

And then also include the module for home-manager if you want to automatically configure it:

```nix
home-manager.users.${username} = {
    imports = [
        inputs.hyprdock.homeManagerModules.default
        ./yourconfig.nix
    ];
};
```

For only NixOS you can use the package directly:

```nix
environment.systemPackages = [
    inputs.hyprdock.packages.${system}.default
];
```

If you are using home-manager, you can enable and configure hyprdock via nix directly:

```nix
programs.hyprdock = {
  enable = true;
  settings = {
    # set to whatever monitor you have
    monitor_name = "HDMI-1";
    # Commands are attrsets as well:
    open_bar_command = {
      base = "ironbar";
      args = [];
    };
  };
};
```

Lid switch behavior has to be set as a NixOS option:

```nix
services.logind.lidSwitch = "ignore";
```

### When are specific functions called?

- open_bar_command => used to open new bars on new monitors
- close_bar_command => used to close a bugged eww bar
- reload_bar_command => used to remove graphical errors with eww after re-enabling internal monitor
- suspend_command => used to suspend (on lid close without external monitor)
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

List of IDs:

- MainWindow
- MainBox
- InternalButton
- ExternalButton
- ExtendButton
- MirrorButton
- ExportButton
