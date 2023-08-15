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

use crate::{monitors::hypr_monitors::save_hypr_monitor_data, HyprDock};
pub use gtk::{prelude::*, Button};
use gtk4 as gtk;
use std::rc::Rc;

impl HyprDock {
    pub fn run_gui(&self) {
        let dock1 = Rc::new(self.clone());
        let dock2 = Rc::new(self.clone());
        let app = gtk::Application::builder()
            .application_id("org.dashie.hyprdock")
            .build();

        app.connect_startup(move |_| {
            adw::init().unwrap();
            dock1.load_css();
        });
        app.connect_activate(move |app| {
            let key_event_controller = gtk::EventControllerKey::new();
            let apprc = Rc::new(app.clone());
            let app1 = apprc.clone();
            let app2 = apprc.clone();
            let app3 = apprc.clone();
            let app4 = apprc.clone();
            let app5 = apprc.clone();
            let app6 = apprc.clone();
            let config_ref1 = dock2.clone();
            let config_ref2 = dock2.clone();
            let config_ref3 = dock2.clone();
            let config_ref4 = dock2.clone();
            let config_ref5 = dock2.clone();
            let config_ref6 = dock2.clone();
            let main_box = gtk::Box::builder().name("MainBox").build();
            let external = Button::builder()
                .label("External Monitor only")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .name("ExternalButton")
                .build();
            let internal = Button::builder()
                .label("Internal Monitor only")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .name("InternalButton")
                .build();
            let extend = Button::builder()
                .label("Extend Monitors")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .name("ExtendButton")
                .build();
            let mirror = Button::builder()
                .label("Mirror Monitors")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .name("MirrorButton")
                .build();
            let export = Button::builder()
                .label("Export MonitorConfig")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .name("ExportButton")
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
            export.connect_clicked(move |_mirror| {
                save_hypr_monitor_data(config_ref5.monitor_config_path.clone(), None, None);
                app5.quit();
            });

            main_box.append(&internal);
            main_box.append(&external);
            main_box.append(&extend);
            main_box.append(&mirror);
            main_box.append(&export);

            let window = Rc::new(
                adw::ApplicationWindow::builder()
                    .application(app)
                    .title("Monitor Portal")
                    .content(&main_box)
                    .name("MainWindow")
                    .build(),
            );
            let window_ref = window.clone();

            gtk4_layer_shell::init_for_window(&*window);
            // gtk4_layer_shell::set_keyboard_interactivity(&window, true);
            gtk4_layer_shell::set_keyboard_mode(&*window, gtk4_layer_shell::KeyboardMode::OnDemand);
            gtk4_layer_shell::set_layer(&*window, gtk4_layer_shell::Layer::Overlay);

            key_event_controller.connect_key_pressed(move |_controller, key, _keycode, _state| {
                match key {
                    gtk4::gdk::Key::Escape => {
                        window_ref.close();
                        gtk::Inhibit(true)
                    }
                    gtk4::gdk::Key::_1 => {
                        config_ref6.internal_monitor();
                        app6.quit();
                        gtk::Inhibit(true)
                    }
                    gtk4::gdk::Key::_2 => {
                        config_ref6.external_monitor();
                        app6.quit();
                        gtk::Inhibit(true)
                    }
                    gtk4::gdk::Key::_3 => {
                        config_ref6.extend_monitor();
                        app6.quit();
                        gtk::Inhibit(true)
                    }
                    gtk4::gdk::Key::_4 => {
                        config_ref6.mirror_monitor();
                        app6.quit();
                        gtk::Inhibit(true)
                    }
                    gtk4::gdk::Key::_5 => {
                        save_hypr_monitor_data(config_ref6.monitor_config_path.clone(), None, None);
                        app6.quit();
                        gtk::Inhibit(true)
                    }
                    _ => gtk::Inhibit(false),
                }
            });

            window.add_controller(key_event_controller);
            window.present();
        });
        app.run_with_args(&[""]);
    }
    fn load_css(&self) {
        let context_provider = gtk::CssProvider::new();
        if self.css_string != "" {
            context_provider.load_from_path(&self.css_string);
        }

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &context_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
