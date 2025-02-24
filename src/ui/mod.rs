pub mod lists_view;
pub mod new_list_view;

use adw::prelude::*;
use gettextrs::gettext;
use gtk::{self, gdk, gio, glib};

use crate::tr;

pub fn build_ui(app: &adw::Application) {
    println!("Starting build_ui function");

    // Set up icons first
    if let Some(display) = gdk::Display::default() {
        println!("Display found");
        let icon_theme = gtk::IconTheme::for_display(&display);

        // Get the current executable path
        if let Ok(exe_path) = std::env::current_exe() {
            println!("Executable path: {:?}", exe_path);
            if let Some(exe_dir) = exe_path.parent() {
                println!("Executable directory: {:?}", exe_dir);
                // Try multiple possible icon paths relative to the executable
                let possible_paths = [
                    exe_dir.join("data/icons"),
                    exe_dir.join("../data/icons"),
                    std::path::PathBuf::from("./data/icons"),
                    std::path::PathBuf::from("../data/icons"),
                ];

                for path in possible_paths {
                    if path.exists() {
                        println!("Adding icon path: {:?}", path);
                        icon_theme.add_search_path(path);
                    }
                }
            }
        }

        println!("Current search paths: {:?}", icon_theme.search_path());
    } else {
        println!("No display found");
    }

    let header_bar = adw::HeaderBar::new();
    let title = adw::WindowTitle::builder()
        .title(&tr!("Puter"))
        .subtitle(&tr!("Shopping List Manager"))
        .build();

    header_bar.set_title_widget(Some(&title));

    // Add new list button to header bar
    let new_button = gtk::Button::builder()
        .label(&tr!("New List"))
        .css_classes(vec!["suggested-action"])
        .build();

    let main_stack = gtk::Stack::new();
    new_button.connect_clicked(glib::clone!(@weak main_stack => move |_| {
        main_stack.set_visible_child_name("new-list");
    }));

    // Add menu button to header bar
    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .build();

    let menu = gio::Menu::new();
    menu.append(Some(&tr!("About")), Some("app.about"));

    let menu_model = gio::MenuModel::from(menu);
    menu_button.set_menu_model(Some(&menu_model));

    header_bar.pack_end(&menu_button);
    header_bar.pack_end(&new_button);

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(move |_, _| {
        let about_dialog = gtk::AboutDialog::builder()
            .program_name("Puter")
            .logo_icon_name("lol.janjic.puter")
            .version("1.0.0")
            .copyright("© 2025 Ivan Janjić")
            .license_type(gtk::License::MitX11)
            .website("https://janjic.lol")
            .authors(vec![String::from("Ivan Janjić")])
            .build();
        about_dialog.show();
    });
    app.add_action(&about_action);

    println!("Creating lists_view");
    let lists_view = lists_view::build_lists_view(&main_stack);
    println!("lists_view created");

    println!("Creating new_list_view");
    let new_list_view = new_list_view::build_new_list_view(&main_stack, None);
    println!("new_list_view created");

    println!("Adding lists_view to main_stack");
    main_stack.add_named(&lists_view, Some("lists"));
    println!("lists_view added to main_stack");

    println!("Adding new_list_view to main_stack");
    main_stack.add_named(&new_list_view, Some("new-list"));
    println!("new_list_view added to main_stack");

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    content.append(&header_bar);
    content.append(&main_stack);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&tr!("Puter"))
        .default_width(600)
        .default_height(800)
        .icon_name("lol.janjic.puter")
        .build();

    window.set_content(Some(&content));
    window.present();

    println!("UI built and window presented");
}

// Ensure the application runs by connecting to the activate signal
pub fn run_application(app: &adw::Application) {
    app.connect_activate(build_ui);
    app.run();
}