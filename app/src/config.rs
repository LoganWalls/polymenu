use clap::{Args, Parser, ValueHint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tao::dpi::{PhysicalPosition, PhysicalSize};

use crate::command::Command;
use crate::io::IOFormat;

use polymenu_derive::UpdateFromOther;

pub trait UpdateFromOther {
    fn update_from_other(&mut self, other: Self);
}

#[derive(UpdateFromOther, Parser, Serialize, Deserialize, Clone, Default, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(disable_help_flag = true)]
pub struct Config {
    /// Read a config from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    /// Path to your svelte project root for this app
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub app_src: Option<PathBuf>,

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

    /// Configuration options related to the webview window
    #[command(flatten)]
    #[serde(default)]
    pub window: WindowOptions,

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

    #[clap(long, action = clap::ArgAction::HelpLong)]
    #[serde(skip)]
    help: Option<bool>,
}

impl Config {
    pub fn server_url(&self) -> String {
        format!("0.0.0.0:{}", &self.port)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Args, Serialize, Deserialize,
)]
pub struct WindowOptions {
    #[arg(short, long = "window-width", requires = "height")]
    /// Window's width in pixels
    pub width: Option<u32>,

    /// Window's height in pixels
    #[arg(short, long = "window-height", requires = "width")]
    pub height: Option<u32>,

    /// Window's x coordinate in pixels
    #[arg(short, long = "window-x", requires = "y")]
    pub x: Option<u32>,

    /// Window's y coordinate in pixels
    #[arg(short, long = "window-y", requires = "x")]
    pub y: Option<u32>,

    /// Whether or not to use an opaque window (default is transparent)
    #[arg(long = "window-opaque")]
    #[serde(default)]
    pub opaque: bool,

    /// Do not autofocus the window
    #[arg(long = "window-no-focus")]
    #[serde(default)]
    pub no_focus: bool,

    /// Whether or not the window should have decorations
    #[arg(long = "window-decorations")]
    #[serde(default)]
    pub decorations: bool,
}

impl WindowOptions {
    pub fn size(&self) -> Option<PhysicalSize<u32>> {
        if let (Some(width), Some(height)) = (self.width, self.height) {
            Some(PhysicalSize::new(width, height))
        } else {
            None
        }
    }
    pub fn position(&self) -> Option<PhysicalPosition<u32>> {
        if let (Some(x), Some(y)) = (self.x, self.y) {
            Some(PhysicalPosition::new(x, y))
        } else {
            None
        }
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
