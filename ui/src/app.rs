use leptos::html::Input;
use leptos::*;
use polymenu_common::item::Item;
use polymenu_common::Config;

use crate::backend;
use crate::item::MenuItem;
use crate::keybinds::{register_keybinds, Action};
use crate::resize::fit_window_to_content;

#[component]
pub fn App(cx: Scope, config: Config) -> impl IntoView {
    let using_callback = config.callback.is_some();
    let (query, set_query) = create_signal(cx, config.query.clone());
    let (cursor_position, set_cursor_position) = create_signal::<usize>(cx, 0);
    create_effect(cx, move |_| {
        query();
        set_cursor_position(0);
    });

    let all_items = if config.callback.is_some() {
        create_resource(cx, query, backend::fetch_items)
    } else {
        create_resource(cx, || "".to_string(), backend::fetch_items)
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
        fit_window_to_content();
        items
    };

    let item_index = move |collection: &Vec<Item>, id: usize| {
        collection
            .iter()
            .enumerate()
            .find_map(|(i, item)| if item.id == id { Some(i) } else { None })
            .expect("Tried to deselect with invalid id")
    };
    let select_item = move |id: usize| {
        let mut item = all_items
            .try_update(|items| {
                let items = items
                    .as_mut()
                    .expect("Tried to select before items are available");
                let index = item_index(items, id);
                items.remove(index)
            })
            .expect("Tried to select before items are available");
        item.selected = true;
        let old_item = set_selected_items.try_update(move |selected| {
            let mut old_item = None;
            if selected.len() == config.max {
                old_item = selected.pop().map(|mut old| {
                    old.selected = false;
                    old
                });
            }
            selected.push(item);
            old_item
        });
        if let Some(Some(old)) = old_item {
            all_items.update(|items| {
                items
                    .as_mut()
                    .expect("Tried to deselect before items are available")
                    .push(old)
            });
        }
    };
    let deselect_item = move |id: usize| {
        let index = item_index(&selected_items(), id);
        let mut item = set_selected_items
            .try_update(move |selected| selected.remove(index))
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
        Action::Submit => {
            let selected = selected_items();
            let items = if selected.is_empty() {
                vec![visible_items()[cursor_position()].clone()]
            } else {
                selected
            };
            spawn_local(backend::output_items(items));
        }
        Action::Close => spawn_local(backend::close(1)),
    };
    register_keybinds(execute_action);

    let query_ref = create_node_ref::<Input>(cx);
    let rendered_items = move || {
        visible_items()
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                view! { cx,
                    <MenuItem
                        item
                        under_cursor=(i == cursor_position())
                    />
                }
            })
            .collect_view(cx)
    };
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
