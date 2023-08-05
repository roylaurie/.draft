mod custom_window;

use gtk::prelude::*;
use gtk::{self, glib, gio, Application, Switch};
use gio::Settings;
use custom_window::CustomWindow;

const APP_ID: &str = "org.gtk_rs.SavingWindowState1";
const SETTING_SWITCH_ENABLED: &str = "switch-enabled";
const SETTING_WINDOW_WIDTH: &str = "window-width";
const SETTING_WINDOW_HEIGHT: &str = "window-height";
const SETTING_WINDOW_MAXIMIZED: &str = "window-maximized";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let settings = Settings::new(APP_ID);
    settings.bind(SETTING_SWITCH_ENABLED, &switch, "active").build();

    let window = CustomWindow::new(&app);
    window.set_title(Some("GTK-RS Book: Ch 8.1"));
    window.set_child(Some(&switch));

    window.present();
}
