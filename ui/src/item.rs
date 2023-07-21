use std::collections::BTreeMap;

use leptos::*;
use polymenu_common::item::Item;
use polymenu_common::FieldType;

#[component]
pub fn MenuItem(
    cx: Scope,
    item: Item,
    under_cursor: bool,
    extra_fields: Option<BTreeMap<String, FieldType>>,
) -> impl IntoView {
    let content = move || {
        let mut result = Vec::new();
        if let Some(inds) = &item.match_indices {
            let mut label = Vec::new();
            let n_chars = item.data.key.chars().count();
            let mut start = 0;
            let mut chunk_is_match = inds.contains(&start);
            for (i, _) in item.data.key.char_indices() {
                let is_last_char = i == n_chars - 1;
                let char_is_match = inds.contains(&i);
                if char_is_match != chunk_is_match || is_last_char {
                    let end = if is_last_char { i + 1 } else { i };
                    label.push(
                        view! {cx,
                            <span
                                class="item-key"
                                class:matched=move|| chunk_is_match
                            >
                                {item.data.key[start..end].to_string()}
                            </span>
                        }
                        .into_view(cx),
                    );
                    start = i;
                    chunk_is_match = char_is_match;
                }
            }
            result.push(view! {cx, <span class="item-key">{label}</span>}.into_view(cx));
        } else {
            result.push(
                view! {cx, <span data-value={&item.data.key}>{&item.data.key}</span>}.into_view(cx),
            );
        }
        if let Some(extras) = &extra_fields {
            for (name, field_type) in extras.iter() {
                let value = item
                    .data
                    .extra
                    .get(name)
                    .expect(&format!("Could not find extra field: {name}"));
                result.push(match field_type {
                    FieldType::String => view! {cx, <span class=name>{value}</span>}.into_view(cx),
                    FieldType::Image => view! {cx,  <img src=value />}.into_view(cx),
                    FieldType::Url => view! {cx, <iframe src=value></iframe> }.into_view(cx),
                });
            }
        }
        result.collect_view(cx)
    };
    view! {cx,
        <button
            class="item"
            class=("under-cursor", move|| under_cursor)
            class:selected=move|| item.selected
        >
            {content}
        </button>
    }
}
