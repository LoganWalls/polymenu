use clap::{Parser, ValueEnum, ValueHint};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::path::PathBuf;

use crate::keybinds::{Action, Key};

use polymenu_derive::UpdateFromOther;

#[allow(unused)]
trait UpdateFromOther {
    fn update_from_other(&mut self, other: Self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ItemFormat {
    /// CSV without header
    HeadlessCsv,
    /// CSV with header
    Csv,
    /// JSON lines
    Json,
}

impl Default for ItemFormat {
    fn default() -> Self {
        Self::HeadlessCsv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    String,
    Image,
    Url,
}

fn parse_extra_fields(
    input_str: &str,
) -> Result<BTreeMap<String, FieldType>, Box<dyn Error + Send + Sync + 'static>> {
    let mut fields = BTreeMap::new();
    for s in input_str.split(';').map(|s| s.trim()) {
        let (name, field_type) = s
            .split_once(':')
            .ok_or_else(|| String::from("Extra fields types should have the format 'name:type'"))?;
        let field_type: FieldType = serde_json::from_value(field_type.into())?;
        match name {
            "key" => Err("'key' is a reserved name")?,
            "value" => Err("'value' is a reserved name")?,
            other => {
                fields.insert(other.to_string(), field_type);
            }
        }
    }
    Ok(fields)
}

#[derive(UpdateFromOther, Parser, Serialize, Deserialize, Clone, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Execute an external command to populate items whenever the query is changed
    /// String args with the value $QUERY will be set to the current query before
    /// each execution.
    #[arg(last = true, value_name = "COMMAND", verbatim_doc_comment, num_args = 1..)]
    pub callback: Option<Vec<String>>,

    /// How to treat case-sensitivity
    #[arg(long, value_enum, default_value_t = CaseSensitivity::default())]
    #[serde(default)]
    pub case: CaseSensitivity,

    // Specify extra classes for a field
    #[arg(long, value_name = "CLASSES",  num_args = 1..)]
    pub classes: Option<Vec<String>>,

    /// The name of each field (only used when format is headless-csv)
    #[arg(long, value_name = "COLUMN NAMES",  num_args = 1..)]
    pub columns: Option<Vec<String>>,

    /// Read a config from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    /// Specify the type of a field
    #[arg(short, long, value_name = "EXTRA FIELDS", value_parser = parse_extra_fields)]
    pub extra: Option<BTreeMap<String, FieldType>>,

    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// How the items are formated in the input
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<ItemFormat>,

    #[clap(skip)]
    pub keybinds: HashMap<Action, Vec<Key>>,

    /// The maximum number of items that can be selected
    #[arg(short, long, default_value_t = 1)]
    pub max: usize,

    /// The maximum number of items that can be displayed at once
    #[arg(long, default_value_t = 10)]
    pub max_visible: usize,

    /// The prompt to be displayed
    #[arg(short, long, default_value_t = String::from(""))]
    pub prompt: String,

    /// An initial value for the query
    #[arg(short, long, default_value_t = String::from(""))]
    pub query: String,
}
