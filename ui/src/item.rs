use leptos::*;
use polymenu_common::item::Item;

#[component]
pub fn MenuItem(cx: Scope, item: Item, under_cursor: bool) -> impl IntoView {
    let label = move || {
        let mut result = Vec::new();
        if let Some(inds) = &item.match_indices {
            let n_chars = item.data.key.chars().count();
            let mut start = 0;
            let mut chunk_is_match = inds.contains(&start);
            for (i, _) in item.data.key.char_indices() {
                let is_last_char = i == n_chars - 1;
                let char_is_match = inds.contains(&i);
                if char_is_match != chunk_is_match || is_last_char {
                    let end = if is_last_char { i + 1 } else { i };
                    result.push(view! {cx,
                        <span
                            class="item-key"
                            class:matched=move|| chunk_is_match
                        >
                            {item.data.key[start..end].to_string()}
                        </span>
                    });
                    start = i;
                    chunk_is_match = char_is_match;
                }
            }
        } else {
            result.push(view! {cx, <span>{&item.data.key}</span>});
        }
        result.collect_view(cx)
    };
    view! {cx,
        <button
            class="item"
            class=("under-cursor", move|| under_cursor)
            class:selected=move|| item.selected
        >
            {label}
        </button>
    }
}
