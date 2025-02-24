use adw::prelude::*;
use gtk::{self, glib, pango};
use crate::{data::{get_data_dir, load_lists, ShoppingList}, tr, ui::new_list_view::format_currency};

pub fn build_lists_view(stack: &gtk::Stack) -> gtk::Box {
    println!("Starting build_lists_view function");

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

    println!("Total spent: {}", total_spent);
    println!("Total lists: {}", total_lists);
    println!("Average per list: {}", avg_per_list);

    // Create overview labels with frames
    let total_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Total Spent"))
        .halign(gtk::Align::Center) // Align the title to the center
        .build();
    let total_label = gtk::Label::builder()
        .label(&format_currency(total_spent))
        .css_classes(vec!["title-2"])
        .halign(gtk::Align::Center) // Align the label to the center
        .build();
    total_frame.add(&total_label);

    let avg_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Average per List"))
        .halign(gtk::Align::Center) // Align the title to the center
        .build();
    let avg_label = gtk::Label::builder()
        .label(&format_currency(avg_per_list))
        .css_classes(vec!["title-2"])
        .halign(gtk::Align::Center) // Align the label to the center
        .build();
    avg_frame.add(&avg_label);

    let count_frame = adw::PreferencesGroup::builder()
        .title(&tr!("Total Lists"))
        .halign(gtk::Align::Center) // Align the title to the center
        .build();
    let count_label = gtk::Label::builder()
        .label(&total_lists.to_string())
        .css_classes(vec!["title-2"])
        .halign(gtk::Align::Center) // Align the label to the center
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
        println!("Adding list with ID: {}", list.id);
        add_list_row(&lists_box, &list);
    }

    scrolled.set_child(Some(&lists_box));
    box_.append(&scrolled);

    println!("Finished build_lists_view function");
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
        .halign(gtk::Align::Start) // Align the label to the center
        .hexpand(true)
        .build();

    let total = gtk::Label::builder()
        .label(&format!("Total: {}", format_currency(list.total_cost)))
        .halign(gtk::Align::End) // Align the label to the center
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
            .secondary_text(&tr!("Are you sure you want to delete this list? This action cannot be undone."))
            .build();

        if let Some(window) = lists_box.root().and_then(|root| root.downcast::<gtk::Window>().ok()) {
            dialog.set_transient_for(Some(&window));
        }

        dialog.connect_response(glib::clone!(@weak lists_box, @weak row, @strong list => move |dialog, response| {
            if response == gtk::ResponseType::Ok {
                // Delete the file
                let mut path = get_data_dir();
                path.push(format!("{}.json", list.id));
                let _ = std::fs::remove_file(path);

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
        .halign(gtk::Align::Start) // Align the label to the center
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
                        let new_list_view = crate::ui::new_list_view::build_new_list_view(stack, Some(&list));
                        stack.add_named(&new_list_view, Some("edit-list"));
                        stack.set_visible_child_name("edit-list");
                    }
                }
            }
        }
    }));
}