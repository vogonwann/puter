use gettextrs::{gettext, setlocale, textdomain, bindtextdomain, bind_textdomain_codeset, LocaleCategory};
use gtk::glib;
use std::env;
use std::path::Path;

const GETTEXT_PACKAGE: &str = "puter";

pub fn init_i18n() {
    // Print all environment variables related to locale
    println!("LANG: {:?}", env::var("LANG"));
    println!("LC_ALL: {:?}", env::var("LC_ALL"));
    println!("LC_MESSAGES: {:?}", env::var("LC_MESSAGES"));
    println!("LANGUAGE: {:?}", env::var("LANGUAGE"));
    println!("Available languages: {:?}", glib::language_names());

    // Force Serbian locale
    env::set_var("LANGUAGE", "sr_RS:sr");
    env::set_var("LC_ALL", "sr_RS.UTF-8");
    env::set_var("LANG", "sr_RS.UTF-8");

    // Initialize gettext
    setlocale(LocaleCategory::LcAll, "sr_RS.UTF-8");
    textdomain(GETTEXT_PACKAGE).unwrap();

    // Check if translation file exists
    let mo_path = Path::new("/app/share/locale/sr_RS/LC_MESSAGES/puter.mo");
    println!("Translation file exists: {}", mo_path.exists());

    bindtextdomain(GETTEXT_PACKAGE, "/app/share/locale").unwrap();
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").unwrap();

    // Print final environment
    println!("Final LANG: {:?}", env::var("LANG"));
    println!("Final LC_ALL: {:?}", env::var("LC_ALL"));
    println!("Final LANGUAGE: {:?}", env::var("LANGUAGE"));
    println!("Test translation of 'Puter': {}", gettext("Puter"));
    println!("Test translation of 'Add': {}", gettext("Add"));
}

// Helper macro for translations
#[macro_export]
macro_rules! tr {
    ($str:expr) => {
        gettextrs::gettext($str)
    };
}