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
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    process::{Command, Output},
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    match mode.as_str() {
        "extend" => extend_monitor(),
        "mirror" => mirror_monitor(),
        "internal" => internal_monitor(),
        "external" => external_monitor(),
        _ => socket_connect(),
    }
}

fn handle_close() {
    if has_external_monitor() {
        external_monitor();
    } else if !is_charging() {
        stop_music();
        lock_system();
    }
}

fn handle_open() {
    if has_external_monitor() {
        extend_monitor();
    } else if !is_internal_active() {
        internal_monitor();
    }
}

fn handle_event(event: &str) {
    match event {
        "button/lid LID close\n" => handle_close(),
        "button/lid LID open\n" => handle_open(),
        _ => {}
    }
}

fn socket_connect() {
    let mut sock =
        UnixStream::connect("/var/run/acpid.socket").expect("failed to connect to socket");
    loop {
        let mut buf = [0; 1024];
        let n = sock.read(&mut buf).expect("failed to read from socket");
        let data = std::str::from_utf8(&buf[..n]).unwrap().to_string();

        handle_event(data.as_str());
    }
}

fn lock_system() {
    Command::new("waylock")
        .arg("-c")
        .arg("000000")
        .output()
        .expect("Failed to lock screen");
    Command::new("systemctl")
        .arg("suspend")
        .output()
        .expect("Failed to suspend");
}

fn stop_music() {
    Command::new("playerctl")
        .arg("--all-players")
        .arg("-a")
        .arg("pause")
        .output()
        .expect("Failed to pause music players");
}

fn extend_monitor() {
    Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(",highrr,1920x0,1")
        .output()
        .expect("Failed to extend Monitors");
    // .arg("hyprctl keyword eDP-1,1920x1080@144,0x0,1")
}

fn mirror_monitor() {
    Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(",highrr,auto,1,mirror,eDP-1")
        .output()
        .expect("Failed to mirror Monitors");
    // .arg("hyprctl keyword monitor eDP-1,1920x1080@144,0x0,1")
}

fn internal_monitor() {
    Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg("eDP-1,highrr,0x0,1")
        .output()
        .expect("Failed to enable internal monitor");
    Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(",disabled")
        .output()
        .expect("Failed to disable external monitor");
}

fn external_monitor() {
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
}

fn is_charging() -> bool {
    if String::from(
        fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .expect("Should have been able to read the file"),
    ) == "Charging"
    {
        return true;
    }
    false
}

fn is_internal_active() -> bool {
    let output = String::from_utf8(
        Command::new("hyprctl")
            .arg("monitors")
            .output()
            .expect("Failed to use only external monitor")
            .stdout,
    )
    .unwrap();
    if output.contains("eDP-1") {
        return false;
    }
    true
}

fn has_external_monitor() -> bool {
    let output = String::from_utf8(
        Command::new("hyprctl")
            .arg("monitors")
            .output()
            .expect("Failed to use only external monitor")
            .stdout,
    )
    .unwrap();
    if output.contains("ID 1") {
        return false;
    }
    true
}
