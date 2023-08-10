mod task_object;
mod task_row;
mod window;

use gtk::{glib, gio};
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.ch11_1";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ch11_1.gresource").unwrap();
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let window = window::Window::new(app);
    window.present();
}
