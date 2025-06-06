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

use directories_next as dirs;
use monitors::hypr_monitors::{
    get_all_hypr_monitors, get_current_monitor_hash, save_hypr_monitor_data,
    set_hypr_monitors_from_file, try_get_monitor_hash_path,
};
use once_cell::sync::Lazy;
use optional_struct::{Applicable, optional_struct};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::Read,
    os::unix::net::UnixStream,
    path::PathBuf,
    process::{Command, ExitCode},
    thread,
};
use toml;

pub mod gui;
pub mod monitors;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
struct HyprdockCommand {
    base: String,
    args: Vec<String>,
}

impl HyprdockCommand {
    pub fn empty() -> Self {
        Self {
            base: "".into(),
            args: Vec::new(),
        }
    }
    pub fn new(base: &'static str, args: &[&str]) -> Self {
        Self {
            base: base.trim().into(),
            args: args
                .into_iter()
                .map(|val| String::from(*val))
                .collect::<Vec<String>>(),
        }
    }
    pub fn single(base: &'static str) -> Self {
        Self {
            base: base.trim().into(),
            args: Vec::new(),
        }
    }

    pub fn format(&self, monitor: &String) -> Self {
        let mut new_args = Vec::new();
        for arg in self.args.iter() {
            let processed_arg = arg.replace("{}", &monitor);
            new_args.push(processed_arg);
        }
        Self {
            base: self.base.clone(),
            args: new_args,
        }
    }
}

const DEFAULT_CONFIG: Lazy<OptionalHyprDock> = Lazy::new(|| {
    let fetcher = "hyprctl";
    OptionalHyprDock {
        monitor_name: Some("eDP-1".into()),
        default_external_mode: Some("extend".into()),
        open_bar_command: Some(HyprdockCommand::empty()),
        close_bar_command: Some(HyprdockCommand::empty()),
        reload_bar_command: Some(HyprdockCommand::empty()),
        suspend_command: Some(HyprdockCommand::new("systemctl", &["suspend"])),
        lock_command: Some(HyprdockCommand::single("hyprlock")),
        utility_command: Some(HyprdockCommand::empty()),
        get_monitors_command: Some(HyprdockCommand::new(fetcher, &["monitors", "all"])),
        enable_internal_monitor_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", "{},preferred,0x0,1"],
        )),
        disable_internal_monitor_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", "{},disabled"],
        )),
        enable_external_monitor_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", ",preferred,0x0,1"],
        )),
        disable_external_monitor_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", ",disabled"],
        )),
        extend_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", ",preferred,auto,1"],
        )),
        mirror_command: Some(HyprdockCommand::new(
            fetcher,
            &["keyword", "monitor", ",preferred,0x0,1,mirror,{}"],
        )),
        wallpaper_command: Some(HyprdockCommand::empty()),
        css_string: Some("".into()),
        monitor_config_path: Some(
            create_config_dir()
                .unwrap_or_default()
                .to_str()
                .expect("Could not convert path to string")
                .to_string(),
        ),
    }
});

fn default_config_string() -> String {
    toml::to_string(&DEFAULT_CONFIG.to_owned()).unwrap()
}

#[optional_struct]
#[derive(Deserialize, Serialize, Clone)]
struct HyprDock {
    monitor_name: String,
    default_external_mode: String,
    css_string: String,
    monitor_config_path: String,
    open_bar_command: HyprdockCommand,
    close_bar_command: HyprdockCommand,
    reload_bar_command: HyprdockCommand,
    suspend_command: HyprdockCommand,
    lock_command: HyprdockCommand,
    utility_command: HyprdockCommand,
    get_monitors_command: HyprdockCommand,
    enable_internal_monitor_command: HyprdockCommand,
    disable_internal_monitor_command: HyprdockCommand,
    enable_external_monitor_command: HyprdockCommand,
    disable_external_monitor_command: HyprdockCommand,
    extend_command: HyprdockCommand,
    mirror_command: HyprdockCommand,
    wallpaper_command: HyprdockCommand,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return ExitCode::FAILURE;
    }

    let dock = parse_config(
        create_config_dir()
            .map(|path| path.join("hyprdock.toml"))
            .unwrap_or_default()
            .to_str()
            .expect("Could not convert path to string"),
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
            "--utility" | "-u" => dock.utility(),
            "--wallpaper" | "-w" => dock.wallpaper(),
            "--export" | "-ex" => {
                let next_token = iter.next();
                if next_token.is_none() {
                    save_hypr_monitor_data(dock.monitor_config_path.clone(), None, None);
                    return ExitCode::SUCCESS;
                }
                if next_token.unwrap().chars().next().unwrap() == '-' {
                    print_help();
                    return ExitCode::FAILURE;
                }
                save_hypr_monitor_data(dock.monitor_config_path.clone(), next_token, None);
                iteration += 1;
            }
            "--import" | "-in" => {
                let next_token = iter.next();
                if next_token.is_none() {
                    set_hypr_monitors_from_file(dock.monitor_config_path.clone(), None, None);
                    return ExitCode::SUCCESS;
                }
                if next_token.unwrap().chars().next().unwrap() == '-' {
                    print_help();
                    return ExitCode::FAILURE;
                }
                set_hypr_monitors_from_file(dock.monitor_config_path.clone(), next_token, None);
                iteration += 1;
                dock.wallpaper();
                dock.reload_bar();
                dock.fix_bar();
            }
            "--server" | "-s" => dock.socket_connect(),
            "--version" | "-v" => println!("{}", env!("CARGO_PKG_VERSION")),
            "--help" | "-h" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "--gui" | "-g" => dock.run_gui(),
            x => {
                println!("Could not parse {}", x);
                print_help();
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}

fn create_config_dir() -> Result<PathBuf, std::io::Error> {
    let maybe_config_dir = dirs::ProjectDirs::from("com", "Xetibo", "hyprdock");
    if maybe_config_dir.is_none() {
        panic!("Could not get config directory");
    }
    let config = maybe_config_dir.unwrap();
    let config_dir = config.config_dir();
    if !config_dir.exists() {
        fs::create_dir(config_dir)?;
    }
    let monitor_config_path = config_dir.join("monitor_configs/");
    if !monitor_config_path.exists() {
        fs::create_dir(config_dir.join("monitor_configs/"))?;
    }
    let metadata = fs::metadata(config_dir);
    if metadata.is_err() {
        panic!("Could not check directory metadata for config file");
    }
    let file_path = config_dir.join("hyprdock.toml");
    if !file_path.exists() {
        fs::File::create(&file_path)?;
    }
    Ok(config_dir.join(""))
}

fn print_help() {
    print!(
        "Possible arguments are:
            --internal/-io: Switch to internal monitor only
            --external/-eo: Switch to external monitor only
            --extend/-e:    Extends monitors
            --mirror/-m:    Mirrors monitors
            --suspend/-su:  Suspend the system
            --utility/-u:   Use utility command
            --wallpaper/-w  Wallpaper command
            --export/-ex:   Export current monitor config
                            optional name for import
                            usage: hyprdock --export configname OR hyprdock --export
            --import/-in:   Import a monitor config
                            optional name for import 
                            usage: hyprdock --import configname OR hyprdock --import
            --server/-s:    daemon version
                            automatically handles actions on laptop lid close and open.
            --gui/-g:       Launch GUI version
            --version/-v:   shows version
            --help/-h:      shows options\n"
    );
}

fn parse_config(path: &str) -> HyprDock {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => default_config_string(),
    };
    let parsed_conf: OptionalHyprDock = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => DEFAULT_CONFIG.to_owned(),
    };
    parsed_conf.build(DEFAULT_CONFIG.to_owned().try_into().unwrap())
}

impl HyprDock {
    pub fn execute_command(&self, command: HyprdockCommand) {
        let base = command.base.trim().to_string();
        if base.is_empty() {
            return;
        }
        thread::spawn(move || {
            Command::new(base)
                .args(command.args)
                .spawn()
                .expect("Could not parse command, please check your toml");
        });
    }

    pub fn handle_close(&self) {
        if self.has_external_monitor() {
            self.execute_command(
                self.disable_internal_monitor_command
                    .format(&self.monitor_name),
            );
            let monitor_hash = get_current_monitor_hash(None);
            let path = try_get_monitor_hash_path(self.monitor_config_path.clone(), &monitor_hash);
            if path.is_some() {
                set_hypr_monitors_from_file(
                    self.monitor_config_path.clone(),
                    None,
                    Some(&monitor_hash),
                );
            } else {
                self.external_monitor();
            }
            self.wallpaper();
            self.reload_bar();
        } else {
            self.utility();
            self.lock_system();
        }
    }

    pub fn handle_open(&self) {
        let monitor_hash = get_current_monitor_hash(None);
        if self.is_internal_active() {
            return;
        }
        self.execute_command(
            self.enable_internal_monitor_command
                .format(&self.monitor_name),
        );
        let path = try_get_monitor_hash_path(self.monitor_config_path.clone(), &monitor_hash);
        if path.is_some() {
            set_hypr_monitors_from_file(
                self.monitor_config_path.clone(),
                None,
                Some(&monitor_hash),
            );
        } else {
            self.add_monitor();
        }
        self.wallpaper();
        self.reload_bar();
        self.fix_bar();
    }

    pub fn handle_event(&self, event: &str) {
        match event {
            _ if event.contains("LID close") => self.handle_close(),
            _ if event.contains("LID open") => self.handle_open(),
            _ if event.contains("VIDEOOUT plug") => {
                let monitor_hash = get_current_monitor_hash(None);
                let path =
                    try_get_monitor_hash_path(self.monitor_config_path.clone(), &monitor_hash);
                if path.is_none() {
                    self.add_monitor();
                    save_hypr_monitor_data(self.monitor_config_path.clone(), None, None);
                } else {
                    set_hypr_monitors_from_file(
                        self.monitor_config_path.clone(),
                        None,
                        Some(&monitor_hash),
                    );
                }
                self.wallpaper();
                self.reload_bar();
                self.fix_bar();
            }
            _ if event.contains("VIDEOOUT unplug") => {
                let monitor_hash = get_current_monitor_hash(None);
                let path =
                    try_get_monitor_hash_path(self.monitor_config_path.clone(), &monitor_hash);
                if path.is_some() {
                    set_hypr_monitors_from_file(
                        self.monitor_config_path.clone(),
                        None,
                        Some(&monitor_hash),
                    );
                    return;
                }
                self.internal_monitor();
                set_hypr_monitors_from_file(
                    self.monitor_config_path.clone(),
                    None,
                    Some(&monitor_hash),
                );
            }
            _ => {}
        }
    }

    pub fn socket_connect(&self) {
        let sock = UnixStream::connect("/var/run/acpid.socket");
        if sock.is_err() {
            println!(
                "Could not connect to acpid socket, do you have the service installed and running?"
            );
            return;
        }
        let mut sock = sock.unwrap();
        loop {
            let mut buf = [0; 1024];
            let n = sock.read(&mut buf).expect("failed to read from socket");
            let data = std::str::from_utf8(&buf[..n]).unwrap().to_string();

            self.handle_event(data.as_str());
        }
    }

    pub fn lock_system(&self) {
        self.execute_command(self.lock_command.format(&self.monitor_name));
        self.execute_command(self.suspend_command.format(&self.monitor_name));
    }

    pub fn utility(&self) {
        self.execute_command(self.utility_command.format(&self.monitor_name));
    }

    pub fn extend_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        self.execute_command(self.extend_command.format(&self.monitor_name));
    }

    pub fn mirror_monitor(&self) {
        if !self.is_internal_active() {
            self.restart_internal();
        }
        self.execute_command(self.mirror_command.format(&self.monitor_name));
    }

    pub fn internal_monitor(&self) {
        let needs_restart = !self.is_internal_active();
        self.execute_command(
            self.enable_internal_monitor_command
                .format(&self.monitor_name),
        );
        self.execute_command(
            self.disable_external_monitor_command
                .format(&self.monitor_name),
        );
        if needs_restart {
            self.reload_bar();
            self.wallpaper();
        }
    }

    pub fn restart_internal(&self) {
        self.execute_command(
            self.enable_internal_monitor_command
                .format(&self.monitor_name),
        );
        self.wallpaper();
        self.reload_bar();
        self.fix_bar();
    }

    pub fn external_monitor(&self) {
        if !self.has_external_monitor() {
            return;
        }
        let needs_restart = !self.is_internal_active();
        self.execute_command(
            self.disable_internal_monitor_command
                .format(&self.monitor_name),
        );
        self.execute_command(
            self.enable_external_monitor_command
                .format(&self.monitor_name),
        );
        if needs_restart {
            self.reload_bar();
            self.wallpaper();
        }
    }

    pub fn wallpaper(&self) {
        self.execute_command(self.wallpaper_command.format(&self.monitor_name));
    }

    pub fn reload_bar(&self) {
        self.execute_command(self.close_bar_command.format(&self.monitor_name));
        self.execute_command(self.open_bar_command.format(&self.monitor_name));
    }

    pub fn fix_bar(&self) {
        self.execute_command(self.reload_bar_command.format(&self.monitor_name));
    }

    pub fn add_monitor(&self) {
        match self.default_external_mode.as_str() {
            "extend" => self.extend_monitor(),
            "mirror" => self.mirror_monitor(),
            _ => (),
        }
    }

    pub fn is_internal_active(&self) -> bool {
        let current_monitors = get_all_hypr_monitors();
        for monitor in current_monitors {
            if monitor.name == self.monitor_name && !monitor.disabled {
                return true;
            }
        }
        false
    }

    pub fn has_external_monitor(&self) -> bool {
        let current_monitors = get_all_hypr_monitors();
        for monitor in current_monitors {
            if monitor.name != self.monitor_name && !monitor.disabled {
                return true;
            }
        }
        false
    }
}
