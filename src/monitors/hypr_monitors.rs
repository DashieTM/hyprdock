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
    collections::hash_map::DefaultHasher, fs::File, hash::Hash, hash::Hasher, io::Write,
    process::Command,
};

use serde::{Deserialize, Serialize};

use super::Monitor;

#[allow(non_snake_case)]
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

impl Hash for HyprMonitor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.description.hash(state);
        self.make.hash(state);
        self.model.hash(state);
        self.serial.hash(state);
    }
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

pub fn get_current_monitor_hash(name: Option<&String>) -> String {
    let monitors: Vec<HyprMonitor> = serde_json::from_str(
        &String::from_utf8(get_hypr_monitor_info()).expect("Could not parse json"),
    )
    .expect("Could not parse json");
    let mut s = DefaultHasher::new();
    for monitor in monitors.iter() {
        monitor.hash(&mut s);
    }
    name.hash(&mut s);
    s.finish().to_string()
}

pub fn save_hypr_monitor_data(path: String, name: Option<&String>, hash: Option<&String>) {
    let monitor_name = get_current_monitor_hash(name);
    let monitor_hash = hash.unwrap_or(&monitor_name);
    let mut file = File::create(path + "monitor_configs/" + &monitor_hash + ".json")
        .expect("Could not open json file");
    file.write_all(&get_hypr_monitor_info())
        .expect("Could not write to file");
}

pub fn import_hypr_data(
    path: String,
    name: Option<&String>,
    hash: Option<&String>,
) -> Vec<Monitor> {
    use std::io::prelude::*;
    let monitor_name = &get_current_monitor_hash(name);
    let monitor_hash = hash.unwrap_or(monitor_name);
    let mut file = File::open(path + "monitor_configs/" + monitor_hash + ".json")
        .expect("Could not read file");
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

pub fn set_hypr_monitors_from_file(path: String, name: Option<&String>, hash: Option<&String>) {
    let monitors = import_hypr_data(path, name, hash);
    for monitor in monitors {
        monitor.enable_hypr_monitor();
    }
}

#[test]
fn import_data_test() {
    use std::fs::File;
    use std::io::prelude::*;
    let file = File::open("example.json");
    if file.is_err() {
        return;
    }
    let mut file = file.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read data from file");

    let p: Vec<HyprMonitor> =
        serde_json::from_str(contents.as_str()).expect("Could not parse json");
    dbg!(p);
}
