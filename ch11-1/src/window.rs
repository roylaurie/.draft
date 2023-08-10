use gtk::{glib, gio};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use crate::task_object::TaskObject;
use crate::task_row::TaskRow;

mod imp {
    use std::cell::RefCell;
    use gtk::{glib, gio};
    use gtk::subclass::prelude::*;

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/org/gtk_rs/ch11_1/window.ui")]
    pub struct Window {
        #[template_child]
        pub entry: gtk::TemplateChild<gtk::Entry>,
        #[template_child]
        pub tasks_list: gtk::TemplateChild<gtk::ListView>,
        pub tasks: RefCell<Option<gio::ListStore>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "TodoWindow";
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

            let obj = self.obj();
            obj.setup_tasks();
            obj.setup_callbacks();
            obj.setup_factory();
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

    fn new_task(&self) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        } 

        buffer.set_text("");

        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .unwrap()
    }

    fn setup_tasks(&self) {
        let model = gio::ListStore::new::<TaskObject>();

        self.imp().tasks.replace(Some(model));

        let selection_model = gtk::NoSelection::new(Some(self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(glib::clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        self.imp().entry.connect_icon_release(
            glib::clone!(@weak self as window => move |_,_| {
                window.new_task();
            }
        ));
    }

    fn setup_factory(&self) {
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let task_row = TaskRow::new();
            list_item.downcast_ref::<gtk::ListItem>().unwrap()
                .set_child(Some(&task_row));
        });

        factory.connect_bind(move |_, list_item| {
            let task_object = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("The item has to be an `TaskObject`.");

            let task_row = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let task_row = list_item
                .downcast_ref::<gtk::ListItem>().unwrap()
                .child()
                .and_downcast::<TaskRow>().unwrap();

            task_row.unbind();
        });

        self.imp().tasks_list.set_factory(Some(&factory));
    }
}