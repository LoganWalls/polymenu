use leptos::html::Input;
use leptos::leptos_dom::console_log;
use leptos::*;
use polymenu_common::item::Item;
use polymenu_common::Config;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::keybinds::{register_keybinds, Action};
use crate::resize::fit_window_to_content;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub async fn invoke_no_args(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "process"], js_name = "exit")]
    async fn close(exitCode: usize);
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
    let using_callback = config.callback.is_some();
    let (query, set_query) = create_signal(cx, config.query.clone());
    let (cursor_position, set_cursor_position) = create_signal::<usize>(cx, 0);
    let all_items = if config.callback.is_some() {
        create_resource(cx, query, fetch_items)
    } else {
        create_resource(cx, || "".to_string(), fetch_items)
    };
    let (selected_items, set_selected_items) = create_signal::<Vec<Item>>(cx, Vec::new());
    let visible_items = move || {
        let mut items = all_items.read(cx).unwrap_or_default();
        if !using_callback {
            if query().is_empty() {
                items.iter_mut().for_each(|item| {
                    item.score = None;
                    item.match_indices = None;
                });
            } else {
                let matcher = crate::matcher::new_matcher(config.case);
                crate::matcher::update_scores(&query(), &matcher, &mut items);
                items.retain(|item| item.score.is_some());
            }
            // Reverse sort so that items with higher score are on top.
            items.sort_by(|a, b| b.cmp(a));
        }
        let n_items = items.len().min(config.max_visible);
        items = items
            .into_iter()
            .take(n_items.saturating_sub(selected_items().len()))
            .chain(selected_items())
            .take(config.max_visible)
            .collect::<Vec<Item>>();
        set_cursor_position(0);
        fit_window_to_content();
        items
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
        set_selected_items.update(move |selected| {
            if selected.len() == config.max {
                selected.pop();
            }
            selected.push(item)
        });
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
        Action::Close => spawn_local(close(1)),
    };

    register_keybinds(execute_action);
    let rendered_items = move || {
        visible_items()
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                view! { cx,
                    <button
                        class=("under-cursor", move|| i == cursor_position())
                        class:selected=move|| item.selected
                    >
                        {item.data.key}
                    </button>
                }
            })
            .collect_view(cx)
    };
    let query_ref = create_node_ref::<Input>(cx);
    view! { cx,
        <main class="container">
            <input
                node_ref=query_ref
                id="query"
                on:input=move |ev| set_query(event_target_value(&ev))
                on:focus=move |_| {
                    if let Some(this) = query_ref.get() {
                        let content_len = this.value().chars().count() as u32;
                        this.set_selection_start(None).unwrap();
                        this.set_selection_end(Some(content_len)).unwrap();
                    }
                }
                placeholder=config.prompt
                value=config.query
                autocomplete="off"
                autofocus
            />
            <div id="results">{rendered_items}</div>
        </main>
    }
}
