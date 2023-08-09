use gtk::{self, glib, gio};

mod imp {
    use gtk::{self, glib};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/org/gtk_rs/ch10_1/window.ui")]
    pub struct Window {
        #[template_child]
        pub button: gtk::TemplateChild<gtk::Button>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MyGtkAppWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            self.button.connect_clicked(move |button| {
                button.set_label("Hello world!");
            });
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
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

