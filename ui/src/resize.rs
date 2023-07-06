use std::time::Duration;

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
    request_animation_frame(|| {
        // `request_animation_frame` by itself sometimes calls the
        // resize function at the wrong time, so `set_timeout` is needed.
        set_timeout(
            || {
                let body = document().body().expect("Could not get body");
                let width = body.client_width().max(10);
                let height = body.client_height().max(10);
                spawn_local(async move {
                    set_size(LogicalSize::new(width, height)).await;
                });
            },
            Duration::from_millis(1),
        )
    });
}
