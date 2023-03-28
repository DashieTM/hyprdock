/*
Copyright © 2023 Fabio Lenherr

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use serde_derive::Deserialize;
use std::{
    env, fs, io::Read, os::unix::net::UnixStream, path::PathBuf, process::Command, thread,
    time::Duration,
};
use toml;

const DEFAULT_CONFIG: &'static str = r#"monitor_name = 'eDP-1'
        open_bar_command = 'eww open bar'
        close_bar_command = 'eww close-all'
        reload_bar_command = 'eww reload'
        suspend_command = 'systemctl suspend'
        lock_command = 'swaylock -c 000000'
        utility_command = 'playerctl --all-players -a pause'
        get_monitors_command = 'hyprctl monitors'
        enable_internal_monitor_command = 'hyprctl keyword monitor eDP-1,highrr,0x0,1'
        disable_internal_monitor_command = 'hyprctl keyword monitor eDP-1,disabled'
        enable_external_monitor_command = 'hyprctl keyword monitor ,highrr,0x0,1'
        disable_external_monitor_command = 'hyprctl keyword monitor ,disabled'
        extend_command = 'hyprctl keyword monitor ,highrr,1920x0,1'
        mirror_command = 'hyprctl keyword monitor ,highrr,0x0,1'
        wallpaper_command = 'hyprctl dispatch hyprpaper'"#;

#[derive(Deserialize)]
struct HyprDock {
    monitor_name: String,
    open_bar_command: String,
    close_bar_command: String,
    reload_bar_command: String,
    suspend_command: String,
    lock_command: String,
    utility_command: String,
    get_monitors_command: String,
    enable_internal_monitor_command: String,
    disable_internal_monitor_command: String,
    enable_external_monitor_command: String,
    disable_external_monitor_command: String,
    extend_command: String,
    mirror_command: String,
    wallpaper_command: String,
}

#[derive(Deserialize)]
struct HyprDockOptional {
    monitor_name: Option<String>,
    open_bar_command: Option<String>,
    close_bar_command: Option<String>,
    reload_bar_command: Option<String>,
    suspend_command: Option<String>,
    lock_command: Option<String>,
    utility_command: Option<String>,
    get_monitors_command: Option<String>,
    enable_internal_monitor_command: Option<String>,
    disable_internal_monitor_command: Option<String>,
    enable_external_monitor_command: Option<String>,
    disable_external_monitor_command: Option<String>,
    extend_command: Option<String>,
    mirror_command: Option<String>,
    wallpaper_command: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    let dock = parse_config(
        home::home_dir()
            .unwrap()
            .join(PathBuf::from(".config/hypr/hyprdock.toml"))
            .to_str()
            .unwrap(),
    );

    let mut iter = args.iter();
    iter.next();
    let mut iteration = 0;
    for _ in 0..args.len() - 1 {
        if iteration == args.len() - 1 {
            break;
        }
        iteration += 1;
        match iter.next().unwrap().as_str() {
            "--internal" | "-i" => dock.internal_monitor(),
            "--external" | "-e" => dock.external_monitor(),
            "--extend" | "-eo" => dock.extend_monitor(),
            "--mirror" | "-io" => dock.mirror_monitor(),
            "--suspend" | "-su" => dock.lock_system(),
            "--server" | "-s" => dock.socket_connect(),
            "--version" | "-v" => println!("0.2.1"),
            "--help" | "-h" => {
                print_help();
                return;
            }
            x => {
                println!("Could not parse {}", x);
                print_help();
                return;
            }
        }
    }
}

fn print_help() {
    print!(
        "Possible arguments are:
            --internal/-io: Switch to internal monitor only
            --external/-eo: Switch to external monitor only
            --extend/-e:    Extends monitors
            --mirror/-m:    Mirrors monitors
            --suspend/-su:  Suspend the system
            --server/-s:    daemon version
                            automatically handles actions on laptop lid close and open.
            --version/-v:   shows version
            --help/-h:      shows options\n"
    );
}

fn parse_config(path: &str) -> HyprDock {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => String::from(DEFAULT_CONFIG),
    };
    let parsed_conf: HyprDockOptional = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => toml::from_str(DEFAULT_CONFIG).unwrap(),
    };
    let parsed_monitor = parsed_conf
        .monitor_name
        .unwrap_or_else(|| String::from("eDP-1"));
    HyprDock {
        monitor_name: parsed_monitor.clone(),
        open_bar_command: parsed_conf
            .open_bar_command
            .unwrap_or_else(|| String::from("eww open bar")),
        close_bar_command: parsed_conf
            .close_bar_command
            .unwrap_or_else(|| String::from("eww close-all")),
        reload_bar_command: parsed_conf
            .reload_bar_command
            .unwrap_or_else(|| String::from("eww reload")),
        suspend_command: parsed_conf
            .suspend_command
            .unwrap_or_else(|| String::from("systemctl suspend")),
        lock_command: parsed_conf
            .lock_command
            .unwrap_or_else(|| String::from("swayloc -c 000000")),
        utility_command: parsed_conf
            .utility_command
            .unwrap_or_else(|| String::from("playerctl --all-players -a pause")),
        get_monitors_command: parsed_conf
            .get_monitors_command
            .unwrap_or_else(|| String::from("hyprctl monitors")),
        enable_internal_monitor_command: parsed_conf
            .enable_internal_monitor_command
            .unwrap_or_else(|| {
                format!(
                    "hyprctl keyword monitor {},highrr,0x0,1",
                    parsed_monitor.clone()
                )
            }),
        disable_internal_monitor_command: parsed_conf
            .disable_internal_monitor_command
            .unwrap_or_else(|| {
                format!(
                    "hyprctl keyword monitor {},disabled",
                    parsed_monitor.clone()
                )
            }),
        enable_external_monitor_command: parsed_conf
            .enable_external_monitor_command
            .unwrap_or_else(|| String::from("hyprctl keyword monitor ,highrr,auto,1")),
        disable_external_monitor_command: parsed_conf
            .disable_external_monitor_command
            .unwrap_or_else(|| String::from("hyprctl keyword monitor ,disabled")),
        extend_command: parsed_conf
            .extend_command
            .unwrap_or_else(|| String::from("hyprctl keyword monitor ,highrr,auto,1")),
        mirror_command: parsed_conf.mirror_command.unwrap_or_else(|| {
            format!(
                "hyprctl keyword monitor ,highrr,auto,1,mirror,{}",
                parsed_monitor
            )
        }),
        wallpaper_command: parsed_conf
            .wallpaper_command
            .unwrap_or_else(|| String::from("hyprctl dispatch exec hyprpaper")),
    }
}

impl HyprDock {
    pub fn execute_command(&self, command: &str) {
        let toml_split: Vec<&str> = command.split(";;").collect();
        for toml_key in toml_split {
            let command_split: Vec<&str> = toml_key.split(" ").collect();
            println!("{}", command_split.len());
            let (first, rest) = command_split.split_first().unwrap();
            if *first == "" {
                return;
            }
            Command::new(first)
                .args(rest)
                .spawn()
                .expect("Could not parse command, please check your toml");
        }
    }

    pub fn execute_command_with_output(&self, command: &str) -> Vec<u8> {
        let command_split: Vec<&str> = command.split(" ").collect();
        let (first, rest) = command_split.split_first().unwrap();
        if *first == "" {
            return Vec::new();
        }
        Command::new(first)
            .args(rest)
            .output()
            .expect("Could not parse command, please check your toml")
            .stdout
    }

    pub fn handle_close(&self) {
        if self.has_external_monitor() {
            self.external_monitor();
            thread::sleep(Duration::from_millis(1000));
            self.restart_hyprpaper();
            self.restart_eww_bar();
        } else {
            self.stop_music();
            self.lock_system();
        }
    }

    pub fn handle_open(&self) {
        if self.is_internal_active() {
            return;
        }
        if !self.has_external_monitor() {
            self.internal_monitor();
            self.restart_hyprpaper();
            self.restart_eww_bar();
            self.fix_eww_bar();
            return;
        } else {
            self.internal_monitor();
            self.extend_monitor();
            self.restart_hyprpaper();
            self.restart_eww_bar();
            self.fix_eww_bar();
        }
    }

    pub fn handle_event(&self, event: &str) {
        match event {
            "button/lid LID close\n" => self.handle_close(),
            "button/lid LID open\n" => self.handle_open(),
            _ => {}
        }
    }

    pub fn socket_connect(&self) {
        let mut sock =
            UnixStream::connect("/var/run/acpid.socket").expect("failed to connect to socket");
        loop {
            let mut buf = [0; 1024];
            let n = sock.read(&mut buf).expect("failed to read from socket");
            let data = std::str::from_utf8(&buf[..n]).unwrap().to_string();

            self.handle_event(data.as_str());
        }
    }

    pub fn lock_system(&self) {
        self.execute_command(self.lock_command.as_str());
        self.execute_command(self.suspend_command.as_str());
    }

    pub fn stop_music(&self) {
        self.execute_command(self.utility_command.as_str());
    }

    pub fn extend_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        self.execute_command(self.extend_command.as_str());
    }

    pub fn mirror_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        self.execute_command(self.mirror_command.as_str());
    }

    pub fn internal_monitor(&self) {
        let needs_restart = !self.is_internal_active();
        self.execute_command(self.enable_internal_monitor_command.as_str());
        self.execute_command(self.disable_external_monitor_command.as_str());
        if needs_restart {
            self.restart_eww_bar();
            self.restart_hyprpaper();
        }
    }

    pub fn restart_internal(&self) {
        self.execute_command(self.enable_internal_monitor_command.as_str());
        self.restart_hyprpaper();
        self.restart_eww_bar();
        self.fix_eww_bar();
    }

    pub fn external_monitor(&self) {
        if !self.has_external_monitor() {
            return;
        }
        let needs_restart = !self.is_internal_active();
        self.execute_command(self.disable_internal_monitor_command.as_str());
        self.execute_command(self.enable_external_monitor_command.as_str());
        if needs_restart {
            self.restart_eww_bar();
            self.restart_hyprpaper();
        }
    }

    pub fn restart_hyprpaper(&self) {
        self.execute_command(self.wallpaper_command.as_str());
    }

    pub fn restart_eww_bar(&self) {
        self.execute_command(self.close_bar_command.as_str());
        self.execute_command(self.open_bar_command.as_str());
    }

    pub fn fix_eww_bar(&self) {
        self.execute_command(self.reload_bar_command.as_str());
    }

    pub fn is_internal_active(&self) -> bool {
        let output =
            String::from_utf8(self.execute_command_with_output(self.get_monitors_command.as_str()))
                .unwrap();
        if output.contains(self.monitor_name.as_str()) {
            return true;
        }
        false
    }

    pub fn has_external_monitor(&self) -> bool {
        let output =
            String::from_utf8(self.execute_command_with_output(self.get_monitors_command.as_str()))
                .unwrap();
        if output.contains("ID 1") {
            return true;
        }
        false
    }
}
