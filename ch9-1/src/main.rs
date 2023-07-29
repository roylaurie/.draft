mod integer_object;

use gtk::{self, glib, gio};
use gtk::prelude::*;
use integer_object::IntegerObject;

const APP_ID: &str = "gtkrsbook.ch8_1";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let list_box = gtk::ListBox::new();

    // the model
    let vector: Vec<IntegerObject> = (0..=10_000).map(IntegerObject::new).collect();
    let model = gio::ListStore::new(IntegerObject::static_type());
    model.extend_from_slice(&vector);

    // the list factory
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label))
    });

    factory.connect_bind(move |_, list_item| {
        let integer_object = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<IntegerObject>()
            .expect("Failed downcast");

        let label = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("No downcast")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("Bad downcast");

        label.set_label(&integer_object.number().to_string());
    });

    // the view
    let selection_model = gtk::SingleSelection::new(Some(model));
    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    window.present();
}