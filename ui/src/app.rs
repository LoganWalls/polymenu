use std::time::Duration;

use leptos::leptos_dom::console_log;
use leptos::*;
use polymenu_common::item::Item;
use polymenu_common::Config;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::keybinds::{register_keybinds, Action};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub async fn invoke_no_args(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "process"])]
    async fn close();
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
    let matcher = crate::matcher::new_matcher(&config.case);
    let (query, set_query) = create_signal(cx, config.query);
    let (cursor_position, set_cursor_position) = create_signal::<usize>(cx, 0);
    let all_items = if config.callback.is_some() {
        create_resource(cx, query, fetch_items)
    } else {
        create_resource(cx, || "".to_string(), fetch_items)
    };

    let (selected_items, set_selected_items) = create_signal::<Vec<Item>>(cx, Vec::new());
    let visible_items = move || {
        let mut items = all_items.read(cx).unwrap_or_default();
        if config.callback.is_none() {
            crate::matcher::update_scores(&query(), &matcher, &mut items);
        }
        let n_items = items.len().min(config.max_visible);
        items
            .into_iter()
            .take(n_items.saturating_sub(selected_items().len()))
            .chain(selected_items())
            .take(config.max_visible)
            .collect::<Vec<Item>>()
    };

    let select_item = move |id: usize| {
        let mut item = all_items
            .try_update(|items| {
                items
                    .as_mut()
                    .expect("Tried to select before items are available")
                    .remove(id)
            })
            .expect("Tried to select before items are available");
        item.selected = true;
        set_selected_items.update(move |selected| selected.push(item));
    };
    let deselect_item = move |id: usize| {
        let selected_idx = selected_items()
            .iter()
            .enumerate()
            .find_map(|(i, item)| if item.id == id { Some(i) } else { None })
            .expect("Tried to deselect with invalid id");
        let mut item = set_selected_items
            .try_update(move |selected| selected.remove(selected_idx))
            .expect("Tried to deselect with invalid id");
        item.selected = false;
        all_items.update(|items| {
            items
                .as_mut()
                .expect("Tried to deselect before items are available")
                .push(item)
        });
    };
    let execute_action = move |action: &Action| match action {
        Action::CursorUp => {
            let i = cursor_position();
            set_cursor_position(if i == 0 {
                visible_items().len() - 1
            } else {
                i - 1
            })
        }
        Action::CursorDown => {
            let i = cursor_position();
            set_cursor_position(if (i + 1) == visible_items().len() {
                0
            } else {
                i + 1
            })
        }
        Action::ToggleSelection => {
            let i = cursor_position();
            let item = &visible_items()[i];
            if item.selected {
                deselect_item(item.id);
            } else {
                select_item(item.id);
            }
        }
        Action::Submit => console_log(
            &selected_items()
                .iter()
                .map(|item| item.data.key.as_str())
                .collect::<Vec<&str>>()
                .join("\n"),
        ),
        Action::Close => spawn_local(close()),
    };

    register_keybinds(execute_action);

    // Resize the window to fit the content whenever the query changes.
    create_effect(cx, move |_| {
        query();
        request_animation_frame(|| {
            // `request_animation_frame` by itself sometimes calls the
            // resize function at the wrong time, so `set_timeout` is needed.
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
