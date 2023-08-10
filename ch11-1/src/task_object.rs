use gtk::{self, glib};

mod imp {
    use std::cell::RefCell;
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use gtk::prelude::*;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::TaskObject)]
    pub struct TaskObject {
        #[property(name = "completed", get, set, type = bool, member = completed)]
        #[property(name = "content", get, set, type = String, member = content)]
        pub data: RefCell<super::TaskData>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskObject {
        const NAME: &'static str = "TodoTaskObject";
        type Type = super::TaskObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TaskObject {}
}

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        glib::Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String
}