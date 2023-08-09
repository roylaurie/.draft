mod window;

use gtk::{self, glib, gio};
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.ch10_1";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ch10_1.gresource").unwrap();
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let window = window::Window::new(app);
    window.present();
}
