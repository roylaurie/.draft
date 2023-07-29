use gtk::{self, glib};

mod imp {
    use std::cell::Cell;
    use gtk::subclass::prelude::ObjectSubclass;
    use gtk::{self, glib};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use glib_macros;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type= super::IntegerObject)]
    pub struct IntegerObject {
        #[property(get, set)]
        number: Cell<i32>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IntegerObject {
        const NAME: &'static str = "MyIntegerObject";
        type Type = super::IntegerObject;
    }

    #[glib_macros::derived_properties]
    impl ObjectImpl for IntegerObject {}
}

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        glib::Object::builder().property("number", number).build()
    }
}