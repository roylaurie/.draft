mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = CustomButton::with_label("Press me!");
    button.set_margin_top(12);
    button.set_margin_end(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}