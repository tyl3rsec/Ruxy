mod window;

use gtk::gio;
use gtk::prelude::*;
use window::Window;
use gio::Settings;

const APP_ID: &str = "org.tyl3rsec.Ruxy";

fn main() {
    // Register and include resources
    gio::resources_register_include!("ruxy.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Initialize settings
    let settings = Settings::new(APP_ID);

    // Create a window and present it
    let window = Window::new(app);
    window.present();
}
