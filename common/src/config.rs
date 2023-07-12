use clap::{Parser, ValueEnum, ValueHint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::keybinds::{Action, Key};
use crate::UpdateFromOther;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum CaseSensitivity {
    /// Case-sensitive only if query contains uppercase characters
    Smart,
    /// Case-sensitive search
    Respect,
    /// Case-insensitive search
    Ignore,
}

impl Default for CaseSensitivity {
    fn default() -> Self {
        Self::Smart
    }
}

#[derive(UpdateFromOther, Parser, Serialize, Deserialize, Clone, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Read a config from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    /// The prompt to be displayed
    #[arg(short, long, default_value_t = String::from(""))]
    pub prompt: String,

    /// An initial value for the query
    #[arg(short, long, default_value_t = String::from(""))]
    pub query: String,

    /// How to treat case-sensitivity
    #[arg(long, value_enum, default_value_t = CaseSensitivity::default())]
    #[serde(default)]
    pub case: CaseSensitivity,

    /// The maximum number of items that can be selected
    #[arg(short, long, default_value_t = 1)]
    pub max: usize,

    /// The maximum number of items that can be displayed at once
    #[arg(long, default_value_t = 10)]
    pub max_visible: usize,

    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Execute an external command to populate items whenever the query is changed
    /// String args with the value $QUERY will be set to the current query before
    /// each execution.
    #[arg(last = true, value_name = "COMMAND", verbatim_doc_comment, num_args = 1..)]
    pub callback: Option<Vec<String>>,

    /// Read style from a CSS file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub style: Option<Vec<PathBuf>>,

    #[clap(skip)]
    pub keybinds: HashMap<Action, Vec<Key>>,
}
