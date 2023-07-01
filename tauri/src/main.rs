// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod callback;
mod item_source;

use clap::{command, Parser, ValueHint};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::item_source::ItemSource;
use polymenu_common::item::Item;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Which config to use
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Execute an external command to populate items whenever the query is changed
    /// String args with the value $QUERY will be set to the current query before
    /// each execution.
    #[arg(last = true, value_name = "COMMAND", verbatim_doc_comment, num_args = 1..)]
    pub callback: Option<Vec<String>>,
}

#[tauri::command]
fn fetch_items(query: &str, item_source: tauri::State<Mutex<ItemSource>>) -> Vec<Item> {
    item_source
        .lock()
        .unwrap()
        .get_items(query)
        .expect("could not read items")
}

fn main() {
    let cli_args = CliArgs::parse();
    let item_source = ItemSource::new(&cli_args);
    tauri::Builder::default()
        .manage(cli_args)
        .manage(Mutex::new(item_source))
        .invoke_handler(tauri::generate_handler![fetch_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
