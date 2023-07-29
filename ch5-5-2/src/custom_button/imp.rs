use std::cell::Cell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use glib::subclass::Signal;
use gtk::glib;
use glib::Properties;
use glib_macros;
use once_cell::sync::Lazy;

// Object holding the state
#[derive(Default, Properties)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    number: Cell<i32>
}

// Central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
#[glib_macros::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
    
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build() ]
        });
        &SIGNALS
    }

}

// Trait shared byall widgets
impl WidgetImpl for CustomButton {}

const MAX_NUMBER: i32 = 8;

// Trait sharedby all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number = self.obj().number() + 1;
        let obj = self.obj();
        if incremented_number == MAX_NUMBER {
            obj.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            obj.set_number(0);
        } else {
           obj.set_number(incremented_number);
        }
    }
}