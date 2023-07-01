use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
extern "C" {
    type LogicalSize;

    #[wasm_bindgen(constructor)]
    fn new(width: i32, height: i32) -> LogicalSize;
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setSize, js_namespace = ["window", "__TAURI__", "window", "appWindow"])]
    async fn set_size(size: LogicalSize);
}

pub fn fit_window_to_content() {
    let body = document().body().expect("Could not get body");
    let width = body.client_width().max(200);
    let height = body.client_height().max(200);
    spawn_local(async move {
        set_size(LogicalSize::new(width, height)).await;
    });
}
