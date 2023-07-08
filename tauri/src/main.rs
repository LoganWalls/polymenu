// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod callback;
mod item_source;

use std::io::{self, Write};
use std::sync::Mutex;

use crate::item_source::ItemSource;
use polymenu_common::item::Item;
use polymenu_common::{Config, Parser};

#[tauri::command]
fn fetch_config(config: tauri::State<Config>) -> Config {
    (*config).clone()
}

#[tauri::command]
fn fetch_items(query: &str, item_source: tauri::State<Mutex<ItemSource>>) -> Vec<Item> {
    item_source
        .lock()
        .unwrap()
        .get_items(query)
        .expect("Could not read items")
}

#[tauri::command]
fn output_items(items: Vec<Item>) {
    io::stdout()
        .write_all(
            (items
                .iter()
                .map(|item| match &item.data.value {
                    Some(value) => value.clone(),
                    None => item.data.key.clone(),
                })
                .collect::<Vec<String>>()
                .join("\n"))
            .as_bytes(),
        )
        .unwrap();
}

fn main() {
    let config = Config::parse();
    let item_source = ItemSource::new(&config);
    tauri::Builder::default()
        .manage(config)
        .manage(Mutex::new(item_source))
        .invoke_handler(tauri::generate_handler![
            fetch_config,
            fetch_items,
            output_items
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
