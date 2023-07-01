use leptos::*;
use polymenu_common::item::Item;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FetchItemsArgs<'a> {
    query: &'a str,
}

async fn fetch_items(query: &str) -> Vec<Item> {
    let args = to_value(&FetchItemsArgs { query }).unwrap();
    from_value::<Vec<Item>>(invoke("fetch_items", args).await).unwrap()
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (query, set_query) = create_signal(cx, "".to_string());
    let all_items = create_resource(cx, || "", fetch_items);
    let (visible_items, set_visible_items) = create_signal::<Vec<Item>>(cx, Vec::new());

    let update_query = move |ev| {
        set_query(event_target_value(&ev));
    };

    create_effect(cx, move |_| {
        set_visible_items.set(
            all_items
                .read(cx)
                .unwrap_or_default()
                .into_iter()
                .filter(|i| i.data.key.contains(&query()))
                .collect(),
        );
        crate::resize::fit_window_to_content();
    });

    view! { cx,
        <main class="container" on:load=|_: ev::Event| crate::resize::fit_window_to_content()>
            <input id="query" on:input=update_query />
            <div id="results">
                 <For
                    each=visible_items
                    key=|item| item.id
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
