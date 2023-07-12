mod app;
mod backend;
mod item;
mod keybinds;
mod matcher;
mod resize;

use crate::app::*;
use crate::backend::{fetch_config, fetch_style};
use leptos::*;

#[component]
fn AppWrapper(cx: Scope) -> impl IntoView {
    let styles = create_resource(cx, || {}, fetch_style);
    create_effect(cx, move |_| {
        if let Some(css_blocks) = styles.read(cx) {
            let head = document().head().expect("Could not find head tag");
            for css in css_blocks {
                mount_to(head.clone().into(), |cx| {
                    view! {cx, <style>{css}</style>}
                });
            }
        }
    });

    let config_resource = create_resource(cx, || {}, fetch_config);
    view! {
        cx,
        <>{move || config_resource.read(cx).map(move |config| view! {cx, <App config/>})}</>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {cx, <AppWrapper/>}
    })
}
