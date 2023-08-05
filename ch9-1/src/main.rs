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
    // the model
    let vector: Vec<IntegerObject> = (0..=10_000).map(IntegerObject::new).collect();
    let model = gio::ListStore::new::<IntegerObject>();
    model.extend_from_slice(&vector);

    // the list factory
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        list_item.set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", gtk::Widget::NONE);
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

    let filter = gtk::CustomFilter::new(move |obj| {
        let integer_object = obj.downcast_ref::<IntegerObject>().unwrap();
        integer_object.number() % 2 == 0
    });

    let filter_model = gtk::FilterListModel::new(Some(model), Some(filter.clone()));
    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let num_lh = obj1.downcast_ref::<IntegerObject>().unwrap().number();
        let num_rh = obj2.downcast_ref::<IntegerObject>().unwrap().number();
        num_rh.cmp(&num_lh).into()
    });

    let sort_model = gtk::SortListModel::new(Some(filter_model), Some(sorter.clone()));

    // the view
    let selection_model = gtk::SingleSelection::new(Some(sort_model));
    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let integer_object = model.item(position)
            .and_downcast::<IntegerObject>()
            .unwrap();

        integer_object.increase_number();
        filter.changed(gtk::FilterChange::Different);
        sorter.changed(gtk::SorterChange::Different);
    });


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