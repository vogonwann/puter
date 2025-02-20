use adw::prelude::*;
use chrono::{DateTime, Local};
use gtk::{glib, gdk, pango};
use gtk::glib::clone;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::env;
use gettextrs::*;
use std::path::Path;

const APP_ID: &str = "lol.janjic.puter";
const GETTEXT_PACKAGE: &str = "puter";

#[derive(Serialize, Deserialize, Clone)]
struct ShoppingItem {
    name: String,
    cost: Option<f64>,
    bought: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct ShoppingList {
    items: Vec<ShoppingItem>,
    created_at: DateTime<Local>,
    total_cost: f64,
    id: String,
}

fn init_i18n() {
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

fn init_styles() -> Option<()> {
    let display = gdk::Display::default()?;
    
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        ".bought-item {
            background-color: alpha(@success_color, 0.15);
            border: 1px solid alpha(@success_color, 0.3);
            border-radius: 6px;
            padding: 6px;
            margin: 3px;
        }"
    );

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Some(())
}

fn main() -> glib::ExitCode {
    // Initialize gettext first
    init_i18n();
    
    // Create and initialize the application
    let application = adw::Application::builder()
        .application_id(APP_ID)
        .build();
    
    // Connect to the activate signal
    application.connect_activate(|app| {
        // Initialize styles after GTK is running
        init_styles();
        // Build the UI
        build_ui(app);
    });

    // Run the application
    application.run()
}

// Helper macro for translations
macro_rules! tr {
    ($str:expr) => {
        gettext($str)
    };
}

fn get_data_dir() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push("shopping-list");
    fs::create_dir_all(&path).expect("Failed to create data directory");
    path
}

fn save_list(list: &ShoppingList) {
    let mut path = get_data_dir();
    path.push(format!("{}.json", list.id));
    let json = serde_json::to_string_pretty(list).expect("Failed to serialize list");
    fs::write(path, json).expect("Failed to save list");
}

fn load_lists() -> Vec<ShoppingList> {
    let path = get_data_dir();
    let mut lists: Vec<ShoppingList> = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(list) = serde_json::from_str(&content) {
                    lists.push(list);
                }
            }
        }
    }
    
    lists.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    lists
}

fn format_currency(amount: f64) -> String {
    // Try to get system locale, fallback to "en-US" if not available
    let locale = glib::language_names()
        .first()
        .map(|l| l.to_string())
        .unwrap_or_else(|| "en-US".to_string());

    // Simple locale-based formatting
    match locale.as_str() {
        l if l.starts_with("de") => format!("{:.2} €", amount.to_string().replace(".", ",")),
        l if l.starts_with("fr") => format!("{:.2} €", amount.to_string().replace(".", ",")),
        l if l.starts_with("ja") => format!("¥{}", (amount.round() as i64).to_string()),
        l if l.starts_with("zh") => format!("¥{:.2}", amount),
        l if l.starts_with("sr") => {
            let formatted = format!("{:.2}", amount);
            let parts: Vec<&str> = formatted.split('.').collect();
            let whole = parts[0].chars().rev()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(".")
                .chars().rev().collect::<String>();
            format!("{},{} RSD", whole, parts.get(1).unwrap_or(&"00"))
        },
        _ => format!("${:.2}", amount)  // Default to US format
    }
}

fn build_ui(app: &adw::Application) {
    // Set up icons first
    if let Some(display) = gdk::Display::default() {
        let icon_theme = gtk::IconTheme::for_display(&display);
        
        // Get the current executable path
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try multiple possible icon paths relative to the executable
                let possible_paths = [
                    exe_dir.join("data/icons"),
                    exe_dir.join("../data/icons"),
                    PathBuf::from("./data/icons"),
                    PathBuf::from("../data/icons"),
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
    }

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(&tr!("Puter"))
        .default_width(600)
        .default_height(800)
        .icon_name("lol.janjic.puter")
        .build();

    let header_bar = adw::HeaderBar::new();
    
    let title = adw::WindowTitle::builder()
        .title("Puter")
        .subtitle("Shopping List Manager")
        .build();
    
    header_bar.set_title_widget(Some(&title));

    // Add new list button to header bar
    let new_button = gtk::Button::builder()
        .label("New List")
        .css_classes(vec!["suggested-action"])
        .build();

    let main_stack = gtk::Stack::new();
    new_button.connect_clicked(glib::clone!(@weak main_stack => move |_| {
        main_stack.set_visible_child_name("new-list");
    }));

    header_bar.pack_end(&new_button);

    let lists_view = build_lists_view(&main_stack);
    let new_list_view = build_new_list_view(&main_stack, None);

    main_stack.add_named(&lists_view, Some("lists"));
    main_stack.add_named(&new_list_view, Some("new-list"));

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    content.append(&header_bar);
    content.append(&main_stack);

    window.set_content(Some(&content));
    window.present();
}

fn build_lists_view(stack: &gtk::Stack) -> gtk::Box {
    let box_ = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Add overview box at the top
    let overview_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .margin_bottom(12)
        .build();

    let lists = load_lists();
    
    // Calculate totals
    let total_spent: f64 = lists.iter().map(|list| list.total_cost).sum();
    let total_lists = lists.len();
    let avg_per_list = if total_lists > 0 {
        total_spent / total_lists as f64
    } else {
        0.0
    };

    // Create overview labels with frames
    let total_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Total Spent"))
        .build();
    let total_label = gtk::Label::builder()
        .label(&format_currency(total_spent))
        .css_classes(vec!["title-2"])
        .build();
    total_frame.add(&total_label);

    let avg_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Average per List"))
        .build();
    let avg_label = gtk::Label::builder()
        .label(&format_currency(avg_per_list))
        .css_classes(vec!["title-2"])
        .build();
    avg_frame.add(&avg_label);

    let count_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Total Lists"))
        .build();
    let count_label = gtk::Label::builder()
        .label(&total_lists.to_string())
        .css_classes(vec!["title-2"])
        .build();
    count_frame.add(&count_label);

    overview_box.append(&total_frame);
    overview_box.append(&avg_frame);
    overview_box.append(&count_frame);

    box_.append(&overview_box);

    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .build();

    let lists_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(vec!["boxed-list"])
        .build();

    // Load and display existing lists
    for list in lists {
        add_list_row(&lists_box, &list);
    }

    scrolled.set_child(Some(&lists_box));
    box_.append(&scrolled);
    box_
}

fn add_list_row(lists_box: &gtk::ListBox, list: &ShoppingList) {
    let row = gtk::ListBoxRow::new();
    let row_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();

    let header = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .build();

    let date = gtk::Label::builder()
        .label(&list.created_at.format("%Y-%m-%d %H:%M").to_string())
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    let total = gtk::Label::builder()
        .label(&format!("Total: {}", format_currency(list.total_cost)))
        .halign(gtk::Align::End)
        .build();

    let edit_button = gtk::Button::builder()
        .icon_name("document-edit-symbolic")
        .css_classes(vec!["flat"])
        .build();

    let delete_button = gtk::Button::builder()
        .icon_name("user-trash-symbolic")
        .css_classes(vec!["flat"])
        .build();

    // Delete button handler
    delete_button.connect_clicked(glib::clone!(@weak lists_box, @weak row, @strong list => move |_| {
        // Show confirmation dialog
        let dialog = gtk::MessageDialog::builder()
            .modal(true)
            .buttons(gtk::ButtonsType::OkCancel)
            .message_type(gtk::MessageType::Warning)
            .text("Delete List")
            .secondary_text("Are you sure you want to delete this list? This action cannot be undone.")
            .build();

        if let Some(window) = lists_box.root().and_then(|root| root.downcast::<gtk::Window>().ok()) {
            dialog.set_transient_for(Some(&window));
        }

        dialog.connect_response(glib::clone!(@weak lists_box, @weak row, @strong list => move |dialog, response| {
            if response == gtk::ResponseType::Ok {
                // Delete the file
                let mut path = get_data_dir();
                path.push(format!("{}.json", list.id));
                let _ = fs::remove_file(path);

                // Remove the row from the list box
                lists_box.remove(&row);

                // Refresh the overview statistics
                if let Some(window) = lists_box.root().and_then(|root| root.downcast::<adw::ApplicationWindow>().ok()) {
                    if let Some(content) = window.content() {
                        if let Some(stack) = content.last_child() {
                            if let Some(stack) = stack.downcast_ref::<gtk::Stack>() {
                                if let Some(old_view) = stack.child_by_name("lists") {
                                    stack.remove(&old_view);
                                }
                                let lists_view = build_lists_view(stack);
                                stack.add_named(&lists_view, Some("lists"));
                                stack.set_visible_child_name("lists");
                            }
                        }
                    }
                }
            }
            dialog.close();
        }));

        dialog.show();
    }));

    header.append(&date);
    header.append(&total);
    header.append(&edit_button);
    header.append(&delete_button);
    row_box.append(&header);

    let items_label = gtk::Label::builder()
        .label(&list.items.iter()
            .map(|item| {
                let status = if item.bought { "✓" } else { "○" };
                let cost = item.cost.map_or("".to_string(), |c| format!(" (${:.2})", c));
                format!("{} {}{}", status, item.name, cost)
            })
            .collect::<Vec<_>>()
            .join(", "))
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .build();

    row_box.append(&items_label);
    row.set_child(Some(&row_box));
    lists_box.append(&row);

    // When edit button is clicked, open the list in edit mode
    edit_button.connect_clicked(glib::clone!(@weak lists_box, @strong list => move |_| {
        if let Some(window) = lists_box.root().and_then(|root| root.downcast::<adw::ApplicationWindow>().ok()) {
            if let Some(content) = window.content() {
                if let Some(stack) = content.last_child() {
                    if let Some(stack) = stack.downcast_ref::<gtk::Stack>() {
                        // Remove existing edit-list view if it exists
                        if let Some(old_view) = stack.child_by_name("edit-list") {
                            stack.remove(&old_view);
                        }
                        
                        // Create a new list view with the existing items
                        let new_list_view = build_new_list_view(stack, Some(&list));
                        stack.add_named(&new_list_view, Some("edit-list"));
                        stack.set_visible_child_name("edit-list");
                    }
                }
            }
        }
    }));
}

fn build_new_list_view(stack: &gtk::Stack, existing_list: Option<&ShoppingList>) -> gtk::Box {
    let box_ = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let entry_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .build();

    let item_entry = gtk::Entry::builder()
        .placeholder_text(&tr!("Add new item..."))
        .hexpand(true)
        .build();

    let add_button = gtk::Button::builder()
        .label(&tr!("Add"))
        .css_classes(vec!["suggested-action"])
        .build();

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(vec!["boxed-list"])
        .build();

    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .build();

    scrolled.set_child(Some(&list_box));

    let items: std::rc::Rc<std::cell::RefCell<Vec<ShoppingItem>>> = 
        match existing_list {
            Some(list) => std::rc::Rc::new(std::cell::RefCell::new(list.items.clone())),
            None => std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
        };

    let created_at = existing_list.map(|list| list.created_at).unwrap_or_else(Local::now);
    let list_id = existing_list.map(|list| list.id.clone()).unwrap_or_else(|| format!("list_{}", created_at.timestamp()));

    // If we have existing items, add them to the list box
    if let Some(list) = existing_list {
        for item in &list.items {
            let row = gtk::ListBoxRow::new();
            // Add bought-item class if the item was previously bought
            if item.bought {
                row.add_css_class("bought-item");
            }
            
            let item_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(6)
                .margin_top(6)
                .margin_bottom(6)
                .margin_start(12)
                .margin_end(12)
                .build();

            let check_button = gtk::CheckButton::builder()
                .active(item.bought)
                .build();
            
            let label = gtk::Label::builder()
                .label(&item.name)
                .halign(gtk::Align::Start)
                .hexpand(true)
                .build();
            
            let cost_entry = gtk::Entry::builder()
                .placeholder_text("Cost...")
                .width_request(100)
                .visible(false)
                .build();

            let cost_label = gtk::Label::builder()
                .label(item.cost.map_or("".to_string(), |c| format!("${:.2}", c)))
                .visible(item.cost.is_some())
                .build();

            let delete_button = gtk::Button::builder()
                .icon_name("user-trash-symbolic")
                .css_classes(vec!["flat"])
                .build();

            // Function to update cost
            let update_cost = {
                let items = items.clone();
                let label = label.clone();
                let cost_entry = cost_entry.clone();
                let cost_label = cost_label.clone();
                
                move || {
                    let cost_text = cost_entry.text().trim().to_string();
                    println!("Raw cost text: {}", cost_text);
                    
                    if !cost_text.is_empty() {
                        // Handle both comma and period as decimal separators
                        let normalized_cost = cost_text.replace(',', ".");
                        println!("Normalized cost: {}", normalized_cost);
                        
                        match normalized_cost.parse::<f64>() {
                            Ok(cost) => {
                                println!("Parsed cost: {}", cost);
                                let mut items = items.borrow_mut();
                                if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
                                    item.cost = Some(cost);
                                    let formatted = format_currency(cost);
                                    println!("Formatted cost: {}", formatted);
                                    cost_label.set_label(&formatted);
                                    cost_label.set_visible(true);
                                    cost_entry.set_visible(false);
                                }
                            },
                            Err(e) => println!("Failed to parse cost: {}", e),
                        }
                    }
                }
            };

            check_button.connect_toggled(glib::clone!(@weak cost_entry, @weak cost_label, @weak items, @weak label, @weak row => move |check| {
                let is_active = check.is_active();
                if is_active {
                    cost_entry.set_visible(true);
                    cost_label.set_visible(false);
                    cost_entry.grab_focus();
                    row.add_css_class("bought-item"); // Add this line
                } else {
                    cost_entry.set_visible(false);
                    cost_label.set_visible(true);
                    row.remove_css_class("bought-item"); // Add this line
                }
                
                let mut items = items.borrow_mut();
                if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
                    item.bought = is_active;
                }
            }));

            cost_entry.connect_activate(glib::clone!(@strong update_cost => move |_| {
                update_cost();
            }));

            let focus_controller = gtk::EventControllerFocus::new();
            focus_controller.connect_leave(glib::clone!(@strong update_cost => move |_| {
                update_cost();
            }));
            cost_entry.add_controller(focus_controller);

            let click_controller = gtk::GestureClick::new();
            cost_label.add_controller(click_controller.clone());
            click_controller.connect_pressed(glib::clone!(@weak cost_entry, @weak cost_label => move |_, _, _, _| {
                cost_entry.set_visible(true);
                cost_label.set_visible(false);
                cost_entry.grab_focus();
            }));

            delete_button.connect_clicked(glib::clone!(@weak list_box, @weak row, @weak items, @weak label => move |_| {
                let mut items = items.borrow_mut();
                if let Some(index) = items.iter().position(|i| i.name == label.label()) {
                    items.remove(index);
                }
                list_box.remove(&row);
            }));

            item_box.append(&check_button);
            item_box.append(&label);
            item_box.append(&cost_entry);
            item_box.append(&cost_label);
            item_box.append(&delete_button);
            
            row.set_child(Some(&item_box));
            list_box.append(&row);
        }
    }

    // Function to add new item
    let add_item = {
        let list_box = list_box.clone();
        let item_entry = item_entry.clone();
        let items = items.clone();
        
        move || {
            let text = item_entry.text();
            if !text.is_empty() {
                let item = ShoppingItem {
                    name: text.to_string(),
                    cost: None,
                    bought: false,
                };
                
                let row = gtk::ListBoxRow::new();
                let item_box = gtk::Box::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .spacing(6)
                    .margin_top(6)
                    .margin_bottom(6)
                    .margin_start(12)
                    .margin_end(12)
                    .build();

                let check_button = gtk::CheckButton::builder()
                    .active(item.bought)
                    .build();
                
                let label = gtk::Label::builder()
                    .label(&item.name)
                    .halign(gtk::Align::Start)
                    .hexpand(true)
                    .build();
                
                let cost_entry = gtk::Entry::builder()
                    .placeholder_text("Cost...")
                    .width_request(100)
                    .visible(false)
                    .build();

                let cost_label = gtk::Label::builder()
                    .label("")
                    .visible(false)
                    .build();

                let delete_button = gtk::Button::builder()
                    .icon_name("user-trash-symbolic")
                    .css_classes(vec!["flat"])
                    .build();

                // Function to update cost
                let update_cost = {
                    let items = items.clone();
                    let label = label.clone();
                    let cost_entry = cost_entry.clone();
                    let cost_label = cost_label.clone();
                    
                    move || {
                        let cost_text = cost_entry.text().trim().to_string();
                        println!("Raw cost text: {}", cost_text);
                        
                        if !cost_text.is_empty() {
                            // Handle both comma and period as decimal separators
                            let normalized_cost = cost_text.replace(',', ".");
                            println!("Normalized cost: {}", normalized_cost);
                            
                            match normalized_cost.parse::<f64>() {
                                Ok(cost) => {
                                    println!("Parsed cost: {}", cost);
                                    let mut items = items.borrow_mut();
                                    if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
                                        item.cost = Some(cost);
                                        let formatted = format_currency(cost);
                                        println!("Formatted cost: {}", formatted);
                                        cost_label.set_label(&formatted);
                                        cost_label.set_visible(true);
                                        cost_entry.set_visible(false);
                                    }
                                },
                                Err(e) => println!("Failed to parse cost: {}", e),
                            }
                        }
                    }
                };

                check_button.connect_toggled(clone!(@weak cost_entry, @weak cost_label, @weak items, @weak label, @weak row => move |check| {
                    let is_active = check.is_active();
                    if is_active {
                        cost_entry.set_visible(true);
                        cost_label.set_visible(false);
                        cost_entry.grab_focus();
                        row.add_css_class("bought-item"); // Add this line
                    } else {
                        cost_entry.set_visible(false);
                        cost_label.set_visible(true);
                        row.remove_css_class("bought-item"); // Add this line
                    }
                    
                    let mut items = items.borrow_mut();
                    if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
                        item.bought = is_active;
                    }
                }));

                cost_entry.connect_activate(glib::clone!(@strong update_cost => move |_| {
                    update_cost();
                }));

                let focus_controller = gtk::EventControllerFocus::new();
                focus_controller.connect_leave(glib::clone!(@strong update_cost => move |_| {
                    update_cost();
                }));
                cost_entry.add_controller(focus_controller);

                let click_controller = gtk::GestureClick::new();
                cost_label.add_controller(click_controller.clone());
                click_controller.connect_pressed(clone!(@weak cost_entry, @weak cost_label => move |_, _, _, _| {
                    cost_entry.set_visible(true);
                    cost_label.set_visible(false);
                    cost_entry.grab_focus();
                }));

                delete_button.connect_clicked(clone!(@weak list_box, @weak row, @weak items, @weak label => move |_| {
                    let mut items = items.borrow_mut();
                    if let Some(index) = items.iter().position(|i| i.name == label.label()) {
                        items.remove(index);
                    }
                    list_box.remove(&row);
                }));

                item_box.append(&check_button);
                item_box.append(&label);
                item_box.append(&cost_entry);
                item_box.append(&cost_label);
                item_box.append(&delete_button);
                
                row.set_child(Some(&item_box));
                list_box.append(&row);
                
                items.borrow_mut().push(item);
                item_entry.set_text("");
                item_entry.grab_focus();
            }
        }
    };

    // Connect Add button
    add_button.connect_clicked(glib::clone!(@strong add_item => move |_| {
        add_item();
    }));

    // Connect Enter key
    item_entry.connect_activate(glib::clone!(@strong add_item => move |_| {
        add_item();
    }));

    entry_box.append(&item_entry);
    entry_box.append(&add_button);
    box_.append(&entry_box);
    box_.append(&scrolled);

    let save_button = gtk::Button::builder()
        .label(&tr!("Save List"))
        .css_classes(vec!["suggested-action"])
        .halign(gtk::Align::End)
        .build();

    save_button.connect_clicked(glib::clone!(@weak stack, @weak items => move |_| {
        let items = items.borrow();
        let total_cost: f64 = items.iter()
            .filter_map(|item| item.cost)
            .sum();
        let list = ShoppingList {
            items: items.to_vec(),
            created_at,
            total_cost,
            id: list_id.clone(),
        };

        // Delete old file if it exists
        let mut old_path = get_data_dir();
        old_path.push(format!("{}.json", list_id));
        let _ = fs::remove_file(&old_path);

        // Save new file
        save_list(&list);
        
        // Remove the edit-list view if it exists
        if let Some(window) = stack.root().and_then(|root| root.downcast::<adw::ApplicationWindow>().ok()) {
            if let Some(content) = window.content() {
                if let Some(stack) = content.last_child() {
                    if let Some(stack) = stack.downcast_ref::<gtk::Stack>() {
                        // Remove existing edit-list view
                        if let Some(old_view) = stack.child_by_name("edit-list") {
                            stack.remove(&old_view);
                        }
                        if let Some(old_view) = stack.child_by_name("lists") {
                            stack.remove(&old_view);
                        }
                        
                        // Create new lists view
                        let lists_view = build_lists_view(stack);
                        stack.add_named(&lists_view, Some("lists"));
                        stack.set_visible_child_name("lists");
                    }
                }
            }
        }
    }));

    box_.append(&save_button);
    box_
}