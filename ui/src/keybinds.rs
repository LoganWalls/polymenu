use std::collections::HashMap;

use leptos::ev::{keydown, KeyboardEvent};
use leptos::window_event_listener;
use polymenu_common::keybinds::{Action, Key};

pub fn register_keybinds(
    keymap: &HashMap<Action, Vec<Key>>,
    execute_action: impl Fn(&Action) + 'static,
) {
    let mut flat_keymap = HashMap::new();
    for (action, keys) in keymap.iter() {
        for key in keys {
            flat_keymap.insert(key.clone(), *action);
        }
    }
    window_event_listener(keydown, move |ev: KeyboardEvent| {
        let code = ev.key_code();
        if ev.is_composing() || code == 229 {
            return;
        }
        let key = Key::from(&ev);
        if let Some(action) = flat_keymap.get(&key) {
            ev.prevent_default();
            execute_action(action);
        }
    });
}
