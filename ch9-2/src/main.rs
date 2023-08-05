use gtk::{self, glib};
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.ch9_2";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let model: gtk::StringList = (0..=100_000).map(|n| n.to_string()).collect();

    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_factory, obj| {
        let list_item: &gtk::ListItem = obj.downcast_ref().unwrap();
        let label = gtk::Label::new(None);
        list_item.set_child(Some(&label));

        list_item.property_expression("item")
            .chain_property::<gtk::StringObject>("string")
            .bind(&label, "label", gtk::Widget::NONE);
    });

    let selection_model = gtk::NoSelection::new(Some(model));

    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&list_view)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .child(&scrolled_window)
        .default_width(640)
        .default_height(480)
        .build();

    window.present();
}
