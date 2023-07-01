use leptos::html::body;
use leptos::leptos_dom::console_log;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use polymenu_common::item::Item;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let all_items: Vec<Item> = ["foo", "bar", "baz"]
        .into_iter()
        .enumerate()
        .map(|(i, key)| Item::new(i as u16, key))
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
        crate::resize::fit_window_to_content();
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
        <main class="container" on:load=|_: ev::Event| crate::resize::fit_window_to_content()>
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
