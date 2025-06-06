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

use std::process::Command;

pub mod hypr_monitors;

pub struct Monitor {
    pub name: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub resolution: String,
    pub refreshrate: String,
    pub offset: String,
    pub scale: String,
    pub transform: String,
    pub vrr: bool,
    pub disabled: bool,
}

impl Monitor {
    pub fn set_resolution(&mut self, new_resolution: String) {
        self.resolution = new_resolution;
    }
    pub fn set_refreshrate(&mut self, new_refreshrate: String) {
        self.refreshrate = new_refreshrate;
    }
    pub fn set_offset(&mut self, new_offset: String) {
        self.offset = new_offset;
    }
    pub fn set_scale(&mut self, new_scale: String) {
        self.scale = new_scale;
    }
    pub fn set_transform(&mut self, new_transform: String) {
        self.transform = new_transform;
    }
    pub fn set_vrr(&mut self, new_vrr: bool) {
        self.vrr = new_vrr;
    }
}

/// Hyprland implementation
impl Monitor {
    pub fn enable_hypr_monitor(&self) {
        let monitor_string = format!(
            "{},{}@{},{},{},transform,{}",
            self.name, self.resolution, self.refreshrate, self.offset, self.scale, self.transform
        );
        Command::new("hyprctl")
            .args(["keyword", "monitor", &monitor_string])
            .spawn()
            .expect("Could not enable specified monitor");
    }
}

#[test]
fn monitor_import() {
    use std::{fs::File, io::Write};
    let output = Command::new("hyprctl")
        .args(["-j", "monitors", "all"])
        .output();

    if output.is_err() {
        println!("hyprctl not found, skipping test");
        return;
    }
    let output = output.unwrap().stdout;
    let mut file = File::create("example.json").expect("Could not open json file");
    assert!(file.write_all(&output).is_ok());
}
