use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};
use glib::{clone, MainContext};

const APP_ID: &str = "org.gtk_rs.MainEventLoop1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        let main_context = MainContext::default();
        main_context.spawn_local(clone!(@weak button =>  async move {
            button.set_sensitive(false);
            glib::timeout_future_seconds(5).await;
            button.set_sensitive(true);
            println!("Awake!");
        }));
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}