use anyhow::{Context, Result};
use clap::{ArgAction, Args, Parser, ValueHint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tao::dpi::{PhysicalPosition, PhysicalSize};

use crate::command::Command;
use crate::expansion::expand_path;
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
    pub src: Option<PathBuf>,

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
    #[serde(default)]
    pub options: HashMap<String, Value>,

    /// A set of directories that will be served to the webview via `/files/{key}`
    #[clap(skip)]
    #[serde(default)]
    pub mount: HashMap<String, PathBuf>,

    /// CLI commands that can be run from the webview using their associated key
    #[clap(skip)]
    #[serde(default)]
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

    // Private properties
    #[arg(long = "option", action = ArgAction::Append, value_name = "KEY=VALUE")]
    #[serde(skip)]
    __options_cli: Vec<String>,

    #[arg(long = "mount", action = ArgAction::Append, value_name = "NAME:PATH")]
    #[serde(skip)]
    __mounts_cli: Vec<String>,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    #[serde(skip)]
    help: Option<bool>,
}

impl Config {
    pub fn gui_target_url(&self) -> String {
        format!(
            "http://localhost:{}",
            if self.develop {
                &self.dev_server_port
            } else {
                &self.port
            }
        )
    }

    pub fn server_url(&self) -> String {
        format!("127.0.0.1:{}", &self.port)
    }

    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let src = std::fs::read_to_string(expand_path(path)?)
            .with_context(|| format!("Coule not read configuration file from: {path:?}"))?;
        Ok(toml::from_str(&src)?)
    }

    pub fn default_path() -> PathBuf {
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        let mut path =
            dirs::config_dir().expect("Could not find config directory for current user");

        #[cfg(target_os = "macos")]
        let mut path = {
            let mut path =
                dirs::home_dir().expect("Could not find home directory for current user");
            path.push(".config");
            path
        };

        path.push("polymenu");
        path.push("config.toml");
        path
    }

    /// Apply CLI overrides for `options` and `mount`
    pub fn apply_cli_overrides(mut self) -> Result<Self> {
        for s in self.__options_cli.iter() {
            let (k, v) = s
                .split_once('=')
                .with_context(|| format!("expected format for options is KEY=VALUE, got: {s}"))?;
            let key = k.parse().with_context(|| format!("invalid key: {k}"))?;
            let val = v.parse().with_context(|| format!("invalid value: {v}"))?;
            self.options.insert(key, val);
        }
        for s in self.__mounts_cli.iter() {
            let (k, v) = s
                .split_once(':')
                .with_context(|| format!("expected format for mounts is NAME:PATH, got: {s}"))?;
            let name = k.parse().with_context(|| format!("invalid key: {k}"))?;
            let path = v.parse().with_context(|| format!("invalid value: {v}"))?;
            self.mount.insert(name, path);
        }
        Ok(self)
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
