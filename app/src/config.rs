use clap::{Parser, ValueHint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::command::{Command, IOFormat};

use polymenu_derive::UpdateFromOther;

pub trait UpdateFromOther {
    fn update_from_other(&mut self, other: Self);
}

#[derive(UpdateFromOther, Parser, Serialize, Deserialize, Clone, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Read a config from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    /// Read items from a file instead of STDIN
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Format of STDIN or file input
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<IOFormat>,

    /// Name of each field (only used when format is headless-csv)
    #[arg(long, value_name = "COLUMN NAMES",  num_args = 1..)]
    pub columns: Option<Vec<String>>,

    /// Options to be passed to the app
    #[clap(skip)]
    pub options: HashMap<String, Value>,

    /// CLI commands that can be run from the webview using their associated key
    #[clap(skip)]
    pub commands: HashMap<String, Command>,

    /// Execute an external script to populate items whenever the query is changed
    /// String args with the value $QUERY will be set to the current query before
    /// each execution.
    #[arg(last = true, value_name = "COMMAND", verbatim_doc_comment, num_args = 1.., value_hint = ValueHint::CommandWithArguments)]
    pub input_script: Option<Vec<String>>,
}
