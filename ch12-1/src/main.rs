
use gtk::{glib,gio};
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.ch12_1";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.set_accels_for_action("yar.close", &["<Ctrl>W"]);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder().application(app).build();

    let actions = gio::SimpleActionGroup::new();
    let action_close = gio::SimpleAction::new("close", None);
    action_close.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));

    window.insert_action_group("yar", Some(&actions));
    actions.add_action(&action_close);

    window.present();
}


