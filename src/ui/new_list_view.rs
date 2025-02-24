use adw::prelude::*;
use gtk::{self, glib, pango};
use crate::data::{ShoppingItem, ShoppingList, save_list, get_data_dir};
use crate::tr;
use crate::styles::init_styles;

pub fn format_currency(cost: f64) -> String {
    format!("${:.2}", cost)
}

fn create_item_row(item: &ShoppingItem, items: std::rc::Rc<std::cell::RefCell<Vec<ShoppingItem>>>) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
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

    let cost_label = gtk::Label::builder()
        .label(&item.cost.map_or("".to_string(), |c| format!("${:.2}", c)))
        .halign(gtk::Align::Start)
        .build();

    let quantity_label = gtk::Label::builder()
        .label(&item.quantity.to_string())
        .halign(gtk::Align::Start)
        .build();

    let total_label = gtk::Label::builder()
        .label(item.cost.map_or("".to_string(), |c| format!("Total: ${:.2}", c * item.quantity as f64)))
        .visible(item.cost.is_some())
        .build();

    let edit_button = gtk::Button::builder()
        .icon_name("document-edit-symbolic")
        .css_classes(vec!["flat"])
        .build();

    let delete_button = gtk::Button::builder()
        .icon_name("user-trash-symbolic")
        .css_classes(vec!["flat"])
        .build();

    let update_cost_and_total = {
        let items = items.clone();
        let label = label.clone();
        let cost_label = cost_label.clone();
        let quantity_label = quantity_label.clone();
        let total_label = total_label.clone();
        
        move |cost_text: String, quantity_text: String| {
            if !cost_text.is_empty() && !quantity_text.is_empty() {
                let normalized_cost = cost_text.replace(',', ".");
                match (normalized_cost.parse::<f64>(), quantity_text.parse::<u32>()) {
                    (Ok(cost), Ok(quantity)) => {
                        let mut items = items.borrow_mut();
                        if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
                            item.cost = Some(cost);
                            item.quantity = quantity;
                            let total = cost * quantity as f64;
                            let formatted = format_currency(total);
                            cost_label.set_label(&format!("${:.2}", cost));
                            quantity_label.set_label(&quantity.to_string());
                            total_label.set_label(&formatted);
                            total_label.set_visible(true);
                        }
                    },
                    _ => (),
                }
            }
        }
    };

    check_button.connect_toggled(glib::clone!(@weak cost_label, @weak total_label, @weak items, @weak label, @weak row => move |check| {
        let is_active = check.is_active();
        if is_active {
            row.add_css_class("bought-item");
        } else {
            row.remove_css_class("bought-item");
        }
        
        let mut items = items.borrow_mut();
        if let Some(item) = items.iter_mut().find(|i| i.name == label.label()) {
            item.bought = is_active;
        }
    }));

    edit_button.connect_clicked(glib::clone!(@weak items, @weak label, @weak cost_label, @weak quantity_label, @weak total_label, @weak row, @strong item_box => move |_| {
        let cost_entry = gtk::Entry::builder()
            .placeholder_text("Cost per piece...")
            .width_request(100)
            .text(&cost_label.label().to_string())
            .visible(true)
            .build();

        let quantity_entry = gtk::Entry::builder()
            .placeholder_text("Quantity...")
            .width_request(100)
            .text(&quantity_label.label().to_string())
            .visible(true)
            .build();

        let save_button = gtk::Button::builder()
            .label("Save")
            .css_classes(vec!["suggested-action"])
            .build();
        save_button.connect_clicked(glib::clone!(@weak row, @strong update_cost_and_total, @strong cost_entry, @strong quantity_entry, @strong item_box => move |_| {
            update_cost_and_total(cost_entry.text().to_string(), quantity_entry.text().to_string());
            row.set_child(Some(&item_box));
        }));

        quantity_entry.connect_activate(glib::clone!(@weak save_button => move |_| {
            save_button.emit_clicked();
        }));

        let edit_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(6)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .build();

        edit_box.append(&cost_entry);
        edit_box.append(&quantity_entry);
        edit_box.append(&save_button);

        row.set_child(Some(&edit_box));
    }));

    delete_button.connect_clicked(glib::clone!(@weak items, @weak label, @weak row => move |_| {
        let mut items = items.borrow_mut();
        if let Some(index) = items.iter().position(|i| i.name == label.label()) {
            items.remove(index);
        }
        row.set_child(None::<&gtk::Widget>);
    }));

    item_box.append(&check_button);
    item_box.append(&label);
    item_box.append(&cost_label);
    item_box.append(&quantity_label);
    item_box.append(&total_label);
    item_box.append(&edit_button);
    item_box.append(&delete_button);
    
    row.set_child(Some(&item_box));
    row
}

pub fn build_new_list_view(stack: &gtk::Stack, existing_list: Option<&ShoppingList>) -> gtk::Box {
    init_styles(); // Add this line to ensure CSS is loaded

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

    let cost_entry = gtk::Entry::builder()
        .placeholder_text(&tr!("Cost per piece..."))
        .width_request(100)
        .build();

    let quantity_entry = gtk::Entry::builder()
        .placeholder_text(&tr!("Quantity..."))
        .width_request(100)
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

    let created_at = existing_list.map(|list| list.created_at).unwrap_or_else(chrono::Local::now);
    let list_id = existing_list.map(|list| list.id.clone()).unwrap_or_else(|| format!("list_{}", created_at.timestamp()));

    if let Some(list) = existing_list {
        for item in &list.items {
            let row = create_item_row(item, items.clone());
            list_box.append(&row);
        }
    }

    let add_item = {
        let list_box = list_box.clone();
        let item_entry = item_entry.clone();
        let cost_entry = cost_entry.clone();
        let quantity_entry = quantity_entry.clone();
        let items = items.clone();
        
        move || {
            let text = item_entry.text();
            let cost_text = cost_entry.text();
            let quantity_text = quantity_entry.text();
            if !text.is_empty() && !cost_text.is_empty() && !quantity_text.is_empty() {
                let cost = cost_text.parse::<f64>().unwrap_or(0.0);
                let quantity = quantity_text.parse::<u32>().unwrap_or(1);
                let item = ShoppingItem {
                    name: text.to_string(),
                    cost: Some(cost),
                    quantity,
                    bought: false,
                };
                
                let row = create_item_row(&item, items.clone());
                list_box.append(&row);
                
                items.borrow_mut().push(item);
                item_entry.set_text("");
                cost_entry.set_text("");
                quantity_entry.set_text("");
                item_entry.grab_focus();
            }
        }
    };

    add_button.connect_clicked(glib::clone!(@strong add_item => move |_| {
        add_item();
    }));

    item_entry.connect_activate(glib::clone!(@strong add_item => move |_| {
        add_item();
    }));

    quantity_entry.connect_activate(glib::clone!(@strong add_item => move |_| {
        add_item();
    }));

    entry_box.append(&item_entry);
    entry_box.append(&cost_entry);
    entry_box.append(&quantity_entry);
    entry_box.append(&add_button);
    box_.append(&entry_box);
    box_.append(&scrolled);

    let button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .halign(gtk::Align::End)
        .build();

    let save_button = gtk::Button::builder()
        .label(&tr!("Save List"))
        .css_classes(vec!["suggested-action"])
        .build();

    save_button.connect_clicked(glib::clone!(@weak stack, @weak items, @strong list_id, @strong created_at => move |_button| {
        let items = items.borrow();
        let total_cost: f64 = items.iter()
            .filter_map(|item| item.cost.map(|cost| cost * item.quantity as f64))
            .sum();
        let list = ShoppingList {
            items: items.to_vec(),
            created_at,
            total_cost,
            id: list_id.clone(),
        };

        let mut old_path = get_data_dir();
        old_path.push(format!("{}.json", list_id));
        let _ = std::fs::remove_file(&old_path);

        save_list(&list);
        
        if let Some(window) = stack.root().and_then(|root| root.downcast::<adw::ApplicationWindow>().ok()) {
            if let Some(content) = window.content() {
                if let Some(stack) = content.last_child() {
                    if let Some(stack) = stack.downcast_ref::<gtk::Stack>() {
                        if let Some(old_view) = stack.child_by_name("edit-list") {
                            stack.remove(&old_view);
                        }
                        if let Some(old_view) = stack.child_by_name("lists") {
                            stack.remove(&old_view);
                        }
                        
                        let lists_view = crate::ui::lists_view::build_lists_view(stack);
                        stack.add_named(&lists_view, Some("lists"));
                        stack.set_visible_child_name("lists");
                    }
                }
            }
        }
    }));

    let cancel_button = gtk::Button::builder()
        .label(&tr!("Cancel"))
        .css_classes(vec!["destructive-action"])
        .build();

    cancel_button.connect_clicked(glib::clone!(@weak stack => move |_| {
        if let Some(window) = stack.root().and_then(|root| root.downcast::<adw::ApplicationWindow>().ok()) {
            if let Some(content) = window.content() {
                if let Some(stack) = content.last_child() {
                    if let Some(stack) = stack.downcast_ref::<gtk::Stack>() {
                        if let Some(old_view) = stack.child_by_name("edit-list") {
                            stack.remove(&old_view);
                        }
                        if let Some(old_view) = stack.child_by_name("lists") {
                            stack.remove(&old_view);
                        }
                        
                        let lists_view = crate::ui::lists_view::build_lists_view(stack);
                        stack.add_named(&lists_view, Some("lists"));
                        stack.set_visible_child_name("lists");
                    }
                }
            }
        }
    }));

    button_box.append(&cancel_button);
    button_box.append(&save_button);

    box_.append(&button_box);

    box_
}