use gtk::glib;

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;

    #[derive(Default)]
    pub struct CustomButton;

    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "MyGtkAppCustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }

    impl ObjectImpl for CustomButton {}
    impl WidgetImpl for CustomButton {}
    impl ButtonImpl for CustomButton {}
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
            gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}