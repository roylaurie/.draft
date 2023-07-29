mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Align, Orientation, Box as GtkBox};

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button1 = CustomButton::new();
    let button2 = CustomButton::new();

    button1.bind_property("number", &button2, "number")
        .bidirectional()
        .sync_create()
        .transform_to(|_, number: i32| {
            let n = number + 1;
            Some(n.to_value())
        })
        .transform_from(|_, number: i32| {
            let n = number - 1;
            Some(n.to_value())
        })
        .build();

    button2.connect_number_notify(|button| {
        println!("Button 2 = {}", button.number());
    });

    // Set up box
    let gtk_box = GtkBox::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button1);
    gtk_box.append(&button2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}