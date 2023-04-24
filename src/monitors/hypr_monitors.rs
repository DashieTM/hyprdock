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

use std::{fs::File, io::Write, process::Command};

use serde_derive::{Deserialize, Serialize};

use crate::HyprDock;

use super::Monitor;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprMonitor {
    id: i64,
    name: String,
    description: String,
    make: String,
    model: String,
    serial: String,
    width: i64,
    height: i64,
    refreshRate: f64,
    x: i64,
    y: i64,
    scale: f64,
    transform: i64,
    vrr: bool,
}

impl HyprMonitor {
    pub fn convert_data(&self) -> Monitor {
        Monitor {
            name: self.name.clone(),
            make: self.make.clone(),
            model: self.model.clone(),
            serial: self.serial.clone(),
            resolution: self.width.to_string() + "x" + &self.height.to_string(),
            refreshrate: (self.refreshRate as i64).to_string(),
            offset: self.x.to_string() + "x" + &self.y.to_string(),
            scale: self.scale.to_string(),
            transform: self.transform.to_string(),
            vrr: self.vrr,
        }
    }
}

pub fn get_hypr_monitor_info() -> Vec<u8> {
    Command::new("hyprctl")
        .args(["-j", "monitors"])
        .output()
        .expect("Could not save output to file")
        .stdout
}

pub(crate) fn compare_and_get_monitor_config(dock: &HyprDock) -> Option<Vec<HyprMonitor>> {
    let mut result = None;
    let current = String::from_utf8(get_hypr_monitor_info()).expect("Could not get json");
    let current_monitors: Vec<HyprMonitor> =
        serde_json::from_str(&current).expect("Could not parse json");
    let mut iteration = 0;
    loop {
        use std::io::prelude::*;
        let file =
            File::open(dock.monitor_config_path.clone() + iteration.to_string().as_str() + ".json");
        if file.is_err() {
            break;
        }
        let mut contents = String::new();
        file.unwrap()
            .read_to_string(&mut contents)
            .expect("Could not read data from file");

        let other_monitors: Vec<HyprMonitor> =
            serde_json::from_str(contents.as_str()).expect("Could not parse json");
        if current_monitors.len() != other_monitors.len() {
            continue;
        }
        let mut current_iter = current_monitors.iter();
        let mut other_iter = other_monitors.iter();
        for i in 1..current_monitors.len() {
            let current_monitor = current_iter.next().unwrap();
            let other_monitor = other_iter.next().unwrap();
            if current_monitor.make != other_monitor.make
                && current_monitor.make != ""
                && other_monitor.make != ""
                && current_monitor.model != other_monitor.model
                && current_monitor.model != ""
                && other_monitor.model != ""
                && current_monitor.serial != other_monitor.serial
                && current_monitor.serial != ""
                && other_monitor.serial != ""
            {
                continue;
            }
            println!("Apllying configuration {i}.");
            result = Some(other_monitors);
            break;
        }
        iteration += 1;
    }
    result
}

pub fn save_hypr_monitor_data(path: String) {
    let mut file = File::create(path).expect("Could not open json file");
    file.write_all(&get_hypr_monitor_info())
        .expect("Could not write to file");
}

pub fn import_hypr_data(path: String) -> Vec<Monitor> {
    use std::io::prelude::*;
    let mut file = File::open(path).expect("Could not read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read data from file");

    let hyprmonitors: Vec<HyprMonitor> =
        serde_json::from_str(contents.as_str()).expect("Could not parse json");
    let mut monitors = Vec::new();
    for monitor in hyprmonitors {
        monitors.push(monitor.convert_data());
    }
    monitors
}

pub fn set_hypr_monitors_from_hyprvec(monitors: Vec<HyprMonitor>) {
    for monitor in monitors {
        let new_monitor = monitor.convert_data();
        new_monitor.enable_hypr_monitor();
    }
}

pub fn set_hypr_monitors_from_file(path: String) {
    let monitors = import_hypr_data(path);
    for monitor in monitors {
        monitor.enable_hypr_monitor();
    }
}

#[test]
fn import_data_test() {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open("example.json").expect("Could not read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read data from file");

    let p: Vec<HyprMonitor> =
        serde_json::from_str(contents.as_str()).expect("Could not parse json");
    dbg!(p);
}
