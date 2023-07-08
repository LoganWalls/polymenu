use std::collections::HashMap;

use leptos::ev::{keydown, KeyboardEvent};
use leptos::window_event_listener;

pub enum Action {
    CursorUp,
    CursorDown,
    ToggleSelection,
    Submit,
    Close,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub key: String,
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub superkey: bool,
}

impl From<&Vec<&str>> for Key {
    fn from(value: &Vec<&str>) -> Self {
        let mut key = None;
        let mut shift = false;
        let mut alt = false;
        let mut ctrl = false;
        let mut superkey = false;
        for s in value {
            match s.to_ascii_lowercase().as_str() {
                "shift" => shift = true,
                "alt" => alt = true,
                "ctrl" => ctrl = true,
                "super" => superkey = true,
                keystr => key = Some(keystr.to_string()),
            };
        }
        Key {
            key: key.unwrap(),
            shift,
            alt,
            ctrl,
            superkey,
        }
    }
}

impl From<&KeyboardEvent> for Key {
    fn from(value: &KeyboardEvent) -> Self {
        Key {
            key: value.key().to_ascii_lowercase(),
            shift: value.shift_key(),
            alt: value.alt_key(),
            ctrl: value.ctrl_key(),
            superkey: value.meta_key(),
        }
    }
}

pub fn register_keybinds(execute_action: impl Fn(&Action) + 'static) {
    let keymap: HashMap<Key, _> = HashMap::from([
        ((&vec!["ctrl", "j"]).into(), Action::CursorDown),
        ((&vec!["ctrl", "k"]).into(), Action::CursorUp),
        ((&vec!["tab"]).into(), Action::ToggleSelection),
        ((&vec!["enter"]).into(), Action::Submit),
        ((&vec!["ctrl", "d"]).into(), Action::Close),
        ((&vec!["escape"]).into(), Action::Close),
    ]);
    window_event_listener(keydown, move |ev: KeyboardEvent| {
        let code = ev.key_code();
        if ev.is_composing() || code == 229 {
            return;
        }
        let key = Key::from(&ev);
        if let Some(action) = keymap.get(&key) {
            ev.prevent_default();
            execute_action(action);
        }
    });
}
