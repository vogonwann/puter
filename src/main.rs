mod data;
mod ui;
mod localization;
mod styles;

use adw::prelude::*;
use adw::Application;

const APP_ID: &str = "lol.janjic.puter";

fn main() {
    // Initialize GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}