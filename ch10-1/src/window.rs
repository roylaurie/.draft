use gtk::{self, glib, gio};

mod imp {
    use std::cell::Cell;
    use gtk::{self, glib};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use crate::button;


    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/org/gtk_rs/ch10_1/window.ui")]
    pub struct Window {
        pub number: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MyGtkAppWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            button::CustomButton::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}

    #[gtk::template_callbacks]
    impl Window {
        #[template_callback]
        fn on_button_clicked(&self, button: &button::CustomButton) {
            let num = self.number.get();
            self.number.set(num + 1);
            button.set_label(&num.to_string())
        }
    }
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

