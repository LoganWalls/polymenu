mod app;
mod backend;
mod item;
mod keybinds;
mod matcher;
mod resize;

use crate::app::*;
use crate::backend::fetch_config;
use leptos::*;

#[component]
fn AppWrapper(cx: Scope) -> impl IntoView {
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
