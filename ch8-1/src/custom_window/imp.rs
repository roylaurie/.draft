use std::cell::OnceCell;
use gtk::{self, glib, gio};
use gio::Settings;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct CustomWindow {
    pub settings: OnceCell<Settings>
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "MyCustomWindow";
    type Type = super::CustomWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for CustomWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {
    fn close_request(&self) -> glib::signal::Propagation {
        self.obj()
            .save_window_size()
            .expect("Failed to save settings");

        glib::signal::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for CustomWindow {}
