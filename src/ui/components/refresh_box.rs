use glib::Object;
use gtk::glib;

use std::cell::Cell;
use std::sync::OnceLock;

use glib::subclass::Signal;
use glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct RefreshBox(ObjectSubclass<RefreshBoxImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RefreshBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn emit_refresh_status(&self, new_state: bool) {
        self.emit_by_name::<()>("refresh-status", &[&new_state]);
    }
}

impl Default for RefreshBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Properties, Default)]
#[properties(wrapper_type = RefreshBox)]
pub struct RefreshBoxImpl {
    #[property(get, set)]
    needs_refresh: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for RefreshBoxImpl {
    const NAME: &'static str = "RefreshBox";
    type Type = RefreshBox;
    type ParentType = gtk::Box;
}

#[glib::derived_properties]
impl ObjectImpl for RefreshBoxImpl {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![Signal::builder("refresh-status")
                .param_types([bool::static_type()])
                .build()]
        })
    }
}

impl WidgetImpl for RefreshBoxImpl {}

impl BoxImpl for RefreshBoxImpl {}
