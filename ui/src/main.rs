mod app;
mod keybinds;
mod matcher;
mod resize;

use app::invoke_no_args;
use app::*;
use leptos::*;
use polymenu_common::Config;
use serde_wasm_bindgen::from_value;

async fn fetch_config(_: ()) -> Config {
    from_value::<Config>(invoke_no_args("fetch_config").await).unwrap()
}

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
