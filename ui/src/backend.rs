use std::path::PathBuf;

use polymenu_common::item::Item;
use polymenu_common::{Config, ImageData};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub async fn invoke_no_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "process"], js_name = "exit")]
    pub async fn close(exitCode: usize);
}

#[derive(Serialize, Deserialize)]
struct FetchItemsArgs<'a> {
    query: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FetchImageArgs {
    path: PathBuf,
}

pub async fn fetch_items(query: String) -> Vec<Item> {
    let args = to_value(&FetchItemsArgs { query: &query }).unwrap();
    from_value::<Vec<Item>>(invoke("fetch_items", args).await).unwrap()
}

pub async fn fetch_config(_: ()) -> Config {
    from_value::<Config>(invoke_no_args("fetch_config").await).unwrap()
}

pub async fn fetch_style(_: ()) -> Vec<String> {
    from_value::<Vec<String>>(invoke_no_args("fetch_styles").await).unwrap()
}

pub async fn fetch_image(path: PathBuf) -> ImageData {
    let args = to_value(&FetchImageArgs { path }).unwrap();
    from_value::<ImageData>(invoke("fetch_image", args).await).unwrap()
}

#[derive(Serialize, Deserialize)]
struct OutputItemsArgs {
    items: Vec<Item>,
}
pub async fn output_items(items: Vec<Item>) {
    let args = to_value(&OutputItemsArgs { items }).unwrap();
    invoke("output_items", args).await;
}
