use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

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

#[derive(Serialize, Deserialize)]
struct FetchImageArgs {
    path: PathBuf,
}
static IMG_CACHE: OnceLock<Mutex<HashMap<PathBuf, String>>> = OnceLock::new();

pub async fn fetch_image(path: PathBuf) -> String {
    {
        let cache = IMG_CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap();
        if let Some(img) = cache.get(&path) {
            return img.clone();
        }
    }
    let args = to_value(&FetchImageArgs { path: path.clone() }).unwrap();
    let img = from_value::<ImageData>(invoke("fetch_image", args).await)
        .expect(&format!("Could not load image {}", path.to_string_lossy()))
        .b64_content_string();
    IMG_CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(path, img.clone());
    img
}

#[derive(Serialize, Deserialize)]
struct OutputItemsArgs {
    items: Vec<Item>,
}
pub async fn output_items(items: Vec<Item>) {
    let args = to_value(&OutputItemsArgs { items }).unwrap();
    invoke("output_items", args).await;
}
