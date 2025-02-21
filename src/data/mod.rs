use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShoppingItem {
    pub name: String,
    pub cost: Option<f64>,
    pub quantity: u32,
    pub bought: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShoppingList {
    pub id: String,
    pub items: Vec<ShoppingItem>,
    pub created_at: DateTime<Local>,
    pub total_cost: f64,
}

pub fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("puter");
    path
}

pub fn load_lists() -> Vec<ShoppingList> {
    println!("Starting load_lists function");
    let mut lists = Vec::new();
    let data_dir = get_data_dir();
    println!("Data directory: {:?}", data_dir);

    if let Ok(entries) = fs::read_dir(&data_dir) {
        let mut found_files = false;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                println!("Found file: {:?}", path);
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    found_files = true;
                    println!("Reading file: {:?}", path);
                    if let Ok(contents) = fs::read_to_string(&path) {
                        println!("File contents: {}", contents);
                        if let Ok(list) = serde_json::from_str::<ShoppingList>(&contents) {
                            println!("Parsed list: {:?}", list);
                            lists.push(list);
                        } else {
                            println!("Failed to parse list from file: {:?}", path);
                        }
                    } else {
                        println!("Failed to read file: {:?}", path);
                    }
                }
            }
        }
        if !found_files {
            println!("No JSON files found in data directory");
        }
    } else {
        println!("Failed to read data directory: {:?}", data_dir);
    }

    println!("Finished load_lists function with {} lists loaded", lists.len());
    lists
}

pub fn save_list(list: &ShoppingList) {
    println!("Starting save_list function for list ID: {}", list.id);
    let data_dir = get_data_dir();
    println!("Data directory: {:?}", data_dir);

    if fs::create_dir_all(&data_dir).is_ok() {
        let mut path = data_dir;
        path.push(format!("{}.json", list.id));
        println!("Saving list to file: {:?}", path);

        if let Ok(contents) = serde_json::to_string_pretty(list) {
            if fs::write(&path, contents).is_ok() {
                println!("List saved successfully");
            } else {
                println!("Failed to write list to file: {:?}", path);
            }
        } else {
            println!("Failed to serialize list");
        }
    } else {
        println!("Failed to create data directory: {:?}", data_dir);
    }
}
