use std::time::Duration;

use leptos::*;
use polymenu_common::item::Item;
use polymenu_common::Config;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub async fn invoke_no_args(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FetchItemsArgs<'a> {
    query: &'a str,
}

async fn fetch_items(query: String) -> Vec<Item> {
    let args = to_value(&FetchItemsArgs { query: &query }).unwrap();
    from_value::<Vec<Item>>(invoke("fetch_items", args).await).unwrap()
}

#[component]
pub fn App(cx: Scope, config: Config) -> impl IntoView {
    let (query, set_query) = create_signal(cx, config.query);
    let all_items = if config.callback.is_some() {
        create_resource(cx, query, fetch_items)
    } else {
        create_resource(cx, || "".to_string(), fetch_items)
    };
    let visible_items = move || {
        let items = all_items.read(cx).unwrap_or_default();
        if config.callback.is_some() {
            items
        } else {
            items
                .into_iter()
                .filter(|i| i.data.key.contains(&query()))
                .collect()
        }
    };
    create_effect(cx, move |_| {
        query();
        request_animation_frame(|| {
            // `request_animation_frame` by itself sometimes calls the
            // resize function at the wrong time, so `set_timeout` is
            // needed.
            set_timeout(
                crate::resize::fit_window_to_content,
                Duration::from_millis(1),
            )
        });
    });

    let update_query = move |ev| {
        set_query(event_target_value(&ev));
    };
    view! { cx,
        <main class="container" on:load=|_: ev::Event| crate::resize::fit_window_to_content()>
            <input id="query" on:input=update_query />
            <div id="results">
                 <For
                    each=visible_items
                    key=move |item| item.data.key.clone()
                    // renders each item to a view
                    view=move |cx, item: Item| {
                      view! {
                        cx,
                        <button>{item.data.key}</button>
                      }
                    }
                  />
            </div>
        </main>
    }
}
