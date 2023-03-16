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

use std::{
    env, fs,
    io::{self, BufRead, Read},
    os::unix::net::UnixStream,
    path::PathBuf,
    process::Command,
    thread,
    time::Duration,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print!(
            "Could not parse argument.
            Possible arguments are:
            extend
            mirror
            internal
            external
            server\n"
        );
        return;
    }

    let mode = &args[1];

    let dock = HyprDock {
        monitor_name: parse_monitor_name(),
        bar: String::from(&args[2]),
    };

    match mode.as_str() {
        "extend" => dock.extend_monitor(),
        "mirror" => dock.mirror_monitor(),
        "internal" => dock.internal_monitor(),
        "external" => dock.external_monitor(),
        "server" => dock.socket_connect(),
        _ => print!(
            "Could not parse argument.
            Possible arguments are:
            extend
            mirror
            internal
            external
            server\n"
        ),
    }
}

fn parse_monitor_name() -> String {
    let path = home::home_dir()
        .unwrap()
        .join(PathBuf::from(".config/hypr/hyprland.conf"));
    let file = fs::File::open(path.to_str().unwrap())
        .expect("Could not open hyprland config, make sure it exists.");
    for line in io::BufReader::new(file).lines() {
        if line.as_ref().unwrap().contains("monitor") {
            let mut name = String::new();
            let mut add_to_name = false;
            for char in line.unwrap().chars() {
                if char == '=' {
                    add_to_name = true;
                } else if char == ',' {
                    return name;
                } else if add_to_name {
                    name.push(char);
                }
            }
        }
    }
    panic!("Could not read name for monitor!");
}

struct HyprDock {
    pub monitor_name: String,
    pub bar: String,
}

impl HyprDock {
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
        Command::new("swaylock")
            .arg("-c")
            .arg("000000")
            .spawn()
            .expect("Failed to lock screen");
        Command::new("systemctl")
            .arg("suspend")
            .output()
            .expect("Failed to suspend");
    }

    pub fn stop_music(&self) {
        Command::new("playerctl")
            .arg("--all-players")
            .arg("-a")
            .arg("pause")
            .output()
            .expect("Failed to pause music players");
    }

    pub fn extend_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(",highrr,auto,1")
            .output()
            .expect("Failed to extend Monitors");
        // .arg("hyprctl keyword eDP-1,1920x1080@144,0x0,1")
    }

    pub fn mirror_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(",highrr,auto,1,mirror,".to_string() + self.monitor_name.as_str())
            .output()
            .expect("Failed to mirror Monitors");
        // .arg("hyprctl keyword monitor eDP-1,1920x1080@144,0x0,1")
    }

    pub fn internal_monitor(&self) {
        let needs_restart = !self.is_internal_active();
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(self.monitor_name.clone() + ",highrr,0x0,1")
            .output()
            .expect("Failed to enable internal monitor");
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(",disabled")
            .output()
            .expect("Failed to disable external monitor");
        if needs_restart {
            self.restart_eww_bar();
            self.restart_hyprpaper();
        }
    }

    pub fn restart_internal(&self) {
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(self.monitor_name.clone() + ",highrr,0x0,1")
            .output()
            .expect("Failed to enable internal monitor");
        self.restart_hyprpaper();
        self.restart_eww_bar();
        self.fix_eww_bar();
    }

    pub fn external_monitor(&self) {
        let needs_restart = !self.is_internal_active();
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg(",highrr,0x0,1")
            .output()
            .expect("Failed to enable external monitor");
        Command::new("hyprctl")
            .arg("keyword")
            .arg("monitor")
            .arg("eDP-1,disabled")
            .output()
            .expect("Failed to disable internal monitor");
        if needs_restart {
            self.restart_eww_bar();
            self.restart_hyprpaper();
        }
    }

    pub fn restart_hyprpaper(&self) {
        Command::new("hyprctl")
            .arg("dispatch")
            .arg("exec")
            .arg("hyprpaper")
            .output()
            .expect("Could not restart hyprpaper");
    }

    pub fn restart_eww_bar(&self) {
        Command::new(self.bar.as_str())
            .arg("close-all")
            .output()
            .expect("could not close eww windows");
        Command::new(self.bar.as_str())
            .arg("open")
            .arg("bar")
            .output()
            .expect("Could not open eww bar");
    }

    pub fn fix_eww_bar(&self) {
        Command::new(self.bar.as_str())
            .arg("reload")
            .output()
            .expect("pingpang");
    }

    pub fn is_internal_active(&self) -> bool {
        let output = String::from_utf8(
            Command::new("hyprctl")
                .arg("monitors")
                .output()
                .expect("Failed to use only external monitor")
                .stdout,
        )
        .unwrap();
        if output.contains(self.monitor_name.as_str()) {
            return true;
        }
        false
    }

    pub fn has_external_monitor(&self) -> bool {
        let output = String::from_utf8(
            Command::new("hyprctl")
                .arg("monitors")
                .output()
                .expect("Failed to use only external monitor")
                .stdout,
        )
        .unwrap();
        if output.contains("ID 1") {
            return true;
        }
        false
    }
}
//#TODO find a way to handle waybar properly
// one that doesn't require you to use 9999x code duplication
