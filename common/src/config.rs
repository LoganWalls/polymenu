use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CaseSensitivity {
    /// Case-insensitive only when query is entirely lowercase
    Smart,
    /// Case-sensitive search
    Respect,
    /// Case-insensitive search
    Ignore,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Prompt to be displayed as a placeholder in the text input box
    pub prompt: String,
    /// Initial value for the text input box
    pub query: String,
    /// How to treat case-sensitivity
    pub case: CaseSensitivity,
    /// Maximum number of items that can be selected
    pub max: usize,
    /// Maximum number of items that can be displayed at once
    pub max_visible: usize,
}
