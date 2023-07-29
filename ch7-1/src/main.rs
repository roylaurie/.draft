use gtk::prelude::*;
use gtk::{self, glib, gio, Application, ApplicationWindow, Switch};
use gio::Settings;

const APP_ID: &str = "org.gtk_rs.Settings1";
const SETTING_SWITCH_ENABLED: &str = "is-switch-enabled";

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

    let window = ApplicationWindow::builder()
        .title("GTK-RS Book: Ch 7.1")
        .child(&switch)
        .application(app)
        .build();

    window.present();
}
