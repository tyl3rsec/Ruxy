use glib::signal::Inhibit;
use gtk::gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Box, SearchEntry, Label};
use once_cell::sync::OnceCell;
use glib::subclass::InitializingObject;
use adw::subclass::prelude::*;
use adw::prelude::*;
use adw::Leaflet;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/tyl3rsec/Ruxy/window.ui")]
pub struct Window {
    #[template_child]
    pub search: TemplateChild<SearchEntry>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    pub settings: OnceCell<Settings>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "RuxyWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Inhibit {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}

// Trait shared by all adwaita applcation windows
impl AdwApplicationWindowImpl for Window {}
