use leptos::html::body;
use leptos::leptos_dom::console_log;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_name = setSize, js_namespace = ["window", "__TAURI__", "window", "appWindow"])]
    async fn set_size(size: LogicalSize);
}

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
extern "C" {
    type LogicalSize;

    #[wasm_bindgen(constructor)]
    fn new(width: i32, height: i32) -> LogicalSize;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item<'a> {
    key: &'a str,
}

fn fit_window_to_content() {
    let body = document().body().expect("Could not get body");
    let width = body.client_width().max(200);
    let height = body.client_height().max(200);
    spawn_local(async move {
        set_size(LogicalSize::new(width, height)).await;
    });
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let all_items: Vec<Item> = ["foo", "bar", "baz"]
        .into_iter()
        .map(|key| Item { key })
        .collect();
    let (visible_items, set_visible_items) = create_signal(cx, all_items.clone());

    let update_items = move |ev| {
        let new_query = event_target_value(&ev);
        set_visible_items.set(
            all_items
                .iter()
                .filter(|i| i.key.contains(&new_query))
                .cloned()
                .collect(),
        );
        fit_window_to_content();
    };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         if name.get().is_empty() {
    //             return;
    //         }
    //
    //         let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! { cx,
        <main class="container">
            <input id="query" on:input=update_items />
            <div id="results">
                 <For
                    each=visible_items
                    key=|item| item.key
                    // renders each item to a view
                    view=move |cx, item: Item| {
                      view! {
                        cx,
                        <button>{item.key}</button>
                      }
                    }
                  />
            </div>
        </main>
    }
}
