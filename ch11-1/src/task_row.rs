use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::task_object::TaskObject;

mod imp {
    use std::cell::RefCell;
    use gtk::glib;
    use gtk::subclass::prelude::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gtk_rs/ch11_1/task_row.ui")]
    pub struct TaskRow {
        #[template_child]
        pub completed_button: gtk::TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub content_label: gtk::TemplateChild<gtk::Label>,
        pub bindings: RefCell<Vec<glib::Binding>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskRow {
        const NAME: &'static str = "TodoTaskRow";
        type Type = super::TaskRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TaskRow {}
    impl WidgetImpl for TaskRow {}
    impl BoxImpl for TaskRow {}
}

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for TaskRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, task_object: &TaskObject) {
        let completed_button = self.imp().completed_button.get();
        let content_label = self.imp().content_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let completed_button_binding = task_object
            .bind_property("completed", &completed_button, "active")
            .bidirectional()
            .sync_create()
            .build();

        bindings.push(completed_button_binding);

        let content_label_binding = task_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();

        bindings.push(content_label_binding);

        let content_label_attr_binding = task_object
            .bind_property("completed", &content_label, "attributes")
            .sync_create()
            .transform_to(|_, active| {
                let attr_list = gtk::pango::AttrList::new();
                if active {
                    let attr = gtk::pango::AttrInt::new_strikethrough(true);
                    attr_list.insert(attr);
                }

                Some(attr_list.to_value())
            })
            .build();

        bindings.push(content_label_attr_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}