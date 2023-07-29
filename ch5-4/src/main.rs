use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation, Switch };

const APP_ID: &str= "org.gtk_rs.GObjectProperties1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let switch1 = Switch::new();
    let switch2 = Switch::new();

    switch1
        .bind_property("active", &switch2, "active")
        .bidirectional()
        .build();

    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    
    gtk_box.append(&switch1);
    gtk_box.append(&switch2);

    ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build()
        .present();
}