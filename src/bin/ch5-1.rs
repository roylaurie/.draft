
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Label, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    let number = Rc::new(Cell::new(0));

    let label_number = Rc::new(RefCell::new(Label::builder()
        .label(number.get().to_string())
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()));

    // Connect callbacks, when a button is clicked `number` will be changed
    let number_copy = number.clone();
    let label_number_copy_increase = label_number.clone();
    button_increase.connect_clicked(move |_| {
        number_copy.set(number_copy.get() + 1);
        label_number_copy_increase.borrow_mut().set_label(&number_copy.get().to_string())
    });

    let label_number_copy_decrease = label_number.clone();
    button_decrease.connect_clicked(move |_| {
        number.set(number.get() - 1);
        label_number_copy_decrease.borrow_mut().set_label(&number.get().to_string())
    });

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();


    gtk_box.append(&button_increase);
    gtk_box.append(&*label_number.borrow());
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}