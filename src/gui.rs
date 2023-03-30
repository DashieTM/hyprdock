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

use crate::HyprDock;
use gtk::{gdk, StyleContext};
pub use gtk::{prelude::*, Button};
use std::rc::Rc;

impl HyprDock {
    pub fn run_gui(&self) {
        let dock = Rc::new(self.clone());
        let app = gtk::Application::builder()
            .application_id("org.dashie.hyprdock")
            .build();

        app.connect_startup(|_| {
            gtk::init().unwrap();
            load_css();
        });
        app.connect_activate(move |app| {
            let apprc = Rc::new(app.clone());
            let app1 = apprc.clone();
            let app2 = apprc.clone();
            let app3 = apprc.clone();
            let app4 = apprc.clone();
            let app5 = apprc.clone();
            let config_ref1 = dock.clone();
            let config_ref2 = dock.clone();
            let config_ref3 = dock.clone();
            let config_ref4 = dock.clone();
            let config_ref5 = dock.clone();
            let main_box = gtk::Box::builder().build();
            let external = Button::builder()
                .label("External Monitor only")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();
            let internal = Button::builder()
                .label("Internal Monitor only")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();
            let extend = Button::builder()
                .label("Extend Monitors")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();
            let mirror = Button::builder()
                .label("Mirror Monitors")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            external.connect_clicked(move |_external| {
                config_ref1.external_monitor();
                app1.quit();
            });
            internal.connect_clicked(move |_internal| {
                config_ref2.internal_monitor();
                app2.quit();
            });
            extend.connect_clicked(move |_extend| {
                config_ref3.extend_monitor();
                app3.quit();
            });
            mirror.connect_clicked(move |_mirror| {
                config_ref4.mirror_monitor();
                app4.quit();
            });

            main_box.add(&internal);
            main_box.add(&external);
            main_box.add(&extend);
            main_box.add(&mirror);

            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("Monitor Portal")
                .child(&main_box)
                .build();

            gtk_layer_shell::init_for_window(&window);
            gtk_layer_shell::set_keyboard_interactivity(&window, true);
            gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

            window.connect_key_press_event(move |window, event| {
                use gdk::keys::constants;
                match event.keyval() {
                    constants::Escape => {
                        window.close();
                        Inhibit(true)
                    }
                    constants::_1 => {
                        config_ref5.internal_monitor();
                        app5.quit();
                        Inhibit(true)
                    }
                    constants::_2 => {
                        config_ref5.external_monitor();
                        app5.quit();
                        Inhibit(true)
                    }
                    constants::_3 => {
                        config_ref5.extend_monitor();
                        app5.quit();
                        Inhibit(true)
                    }
                    constants::_4 => {
                        config_ref5.mirror_monitor();
                        app5.quit();
                        Inhibit(true)
                    }
                    _ => Inhibit(false),
                }
            });

            window.show_all();
        });
        app.run_with_args(&[""]);
    }
}
fn load_css() {
    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().unwrap(),
        &gtk::CssProvider::new(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
