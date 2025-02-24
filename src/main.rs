mod data;
mod ui;
mod localization;
mod styles;

use adw::prelude::*;
use adw::Application;
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use std::env;

const APP_ID: &str = "lol.janjic.puter";
const GETTEXT_PACKAGE: &str = "puter";

fn main() {
    // Initialize gettext
    let locale_dir = "po"; // Adjust this path if necessary
    env::set_var("LOCALEDIR", locale_dir);
    println!("LOCALEDIR set to: {}", locale_dir);

    bindtextdomain(GETTEXT_PACKAGE, locale_dir).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Initialize GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}