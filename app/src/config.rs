use clap::{Parser, ValueHint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::command::Command;
use crate::io::IOFormat;

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
    pub headers: Option<Vec<String>>,

    /// Options to be passed to the app at runtime
    #[clap(skip)]
    pub options: HashMap<String, Value>,

    /// CLI commands that can be run from the webview using their associated key
    #[clap(skip)]
    pub commands: HashMap<String, Command>,

    /// Whether or not to use an opaque window (default is transparent)
    #[arg(long)]
    #[serde(default)]
    pub opaque: bool,

    /// Whether or not the window should have decorations
    #[arg(long)]
    #[serde(default)]
    pub window_decorations: bool,

    /// The port that the server should bind to
    #[arg(short, long, value_name = "PORT", default_value_t = default_port())]
    #[serde(default = "default_port")]
    pub port: String,

    /// Launch in development mode
    #[arg(long)]
    #[serde(skip)]
    pub develop: bool,

    /// The port that the front end development server should bind to
    #[arg(short, long, value_name = "PORT", default_value_t = default_dev_server_port())]
    #[serde(default = "default_dev_server_port")]
    pub dev_server_port: String,

    /// The command to launch the front end development server
    #[clap(skip)]
    #[serde(default = "default_develop_command")]
    pub develop_command: Vec<String>,
}

impl Config {
    pub fn server_url(&self) -> String {
        format!("0.0.0.0:{}", &self.port)
    }
}

fn default_port() -> String {
    "7777".to_string()
}

fn default_dev_server_port() -> String {
    "7778".to_string()
}

fn default_develop_command() -> Vec<String> {
    vec!["pnpm", "run", "dev"]
        .into_iter()
        .map(String::from)
        .collect()
}
