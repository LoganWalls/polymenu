use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    CursorPrevious,
    CursorNext,
    ToggleSelection,
    Submit,
    Close,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(from = "Vec<String>", into = "Vec<String>")]
pub struct Key {
    pub key: String,
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub superkey: bool,
}

impl From<Vec<String>> for Key {
    fn from(value: Vec<String>) -> Self {
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
                key_str => key = Some(key_str.to_string()),
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

impl From<Key> for Vec<String> {
    fn from(val: Key) -> Self {
        let mut result = vec![val.key];
        if val.shift {
            result.push(String::from("shift"));
        }
        if val.alt {
            result.push(String::from("alt"));
        }
        if val.ctrl {
            result.push(String::from("ctrl"));
        }
        if val.superkey {
            result.push(String::from("super"));
        }
        result
    }
}
