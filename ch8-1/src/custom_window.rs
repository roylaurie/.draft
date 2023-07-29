mod imp;

use gtk::{self, glib, gio};
use glib::Object;
use gtk::{Application};
use gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use super::{APP_ID, SETTING_WINDOW_WIDTH, SETTING_WINDOW_HEIGHT, SETTING_WINDOW_MAXIMIZED};

glib::wrapper! {
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl CustomWindow{
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("Unable to set settings");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("Unable to rock and roll")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let size = self.default_size();
        let settings = self.settings();
        settings.set_int(SETTING_WINDOW_WIDTH, size.0)?;
        settings.set_int(SETTING_WINDOW_HEIGHT, size.1)?;
        settings.set_boolean(SETTING_WINDOW_MAXIMIZED, self.is_maximized())?;
        Ok(())
    }

    fn load_window_size(&self) {
        let settings = self.settings();
        self.set_default_size(
            settings.int(SETTING_WINDOW_WIDTH),
            settings.int(SETTING_WINDOW_HEIGHT),
        );

        if settings.boolean(SETTING_WINDOW_MAXIMIZED) {
            self.maximize();
        }
    }
}