use anyhow::{Context, Result};
use clap::ValueEnum;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::command::Command;
use crate::config::Config;
use crate::expansion::expand_path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::path::PathBuf;

static STDIN_CONTENT: Lazy<String> = Lazy::new(|| {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    s
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum IOFormat {
    /// CSV without header (default; will be read as an array of strings per row)
    HeadlessCsv,
    /// CSV with header (will be converted to JSON objects for each row)
    Csv,
    /// JSON
    Json,
    /// JSON lines
    JsonLines,
    /// Raw (will be read as a string)
    Raw,
}

impl Default for IOFormat {
    fn default() -> Self {
        Self::Raw
    }
}

#[derive(Debug, Clone)]
pub enum DataSourceKind {
    StdIn,
    File(PathBuf),
    Command(Command),
}

#[derive(Debug)]
pub struct DataParser {
    kind: DataSourceKind,
    format: IOFormat,
    headers: Option<Vec<String>>,
}

impl DataParser {
    pub fn new(kind: DataSourceKind, format: IOFormat, headers: Option<Vec<String>>) -> Self {
        Self {
            kind,
            format,
            headers,
        }
    }

    pub async fn parse(
        &self,
        args: Option<&HashMap<String, String>>,
        stdin_lines: Option<Vec<String>>,
    ) -> Result<Value> {
        let mut source: Box<dyn io::Read> = match self.kind.clone() {
            DataSourceKind::StdIn => Box::new(Cursor::new(STDIN_CONTENT.as_bytes())),
            DataSourceKind::File(path) => {
                Box::new(File::open(expand_path(&path)?).context("failed to open file")?)
            }
            DataSourceKind::Command(callback) => Box::new(
                callback
                    .call(args, stdin_lines)
                    .await
                    .context("failed to execute callback")?,
            ),
        };
        match self.format {
            IOFormat::HeadlessCsv => read_csv(source, false, self.headers.clone()),
            IOFormat::Csv => read_csv(source, true, self.headers.clone()),
            IOFormat::Json => read_json(source),
            IOFormat::JsonLines => read_jsonlines(source),
            IOFormat::Raw => {
                let mut buf = String::new();
                source
                    .read_to_string(&mut buf)
                    .context("failed to read raw input")?;
                Ok(Value::String(buf))
            }
        }
    }
}

impl From<Config> for DataParser {
    fn from(value: Config) -> Self {
        let format = value.format.unwrap_or_else(|| {
            if let Some(extension) = value
                .file
                .as_deref()
                .and_then(|p| p.extension())
                .and_then(|e| e.to_str())
            {
                match extension {
                    "csv" => IOFormat::Csv,
                    "json" => IOFormat::Json,
                    "jsonl" => IOFormat::JsonLines,
                    _ => IOFormat::Raw,
                }
            } else {
                IOFormat::HeadlessCsv
            }
        });
        let kind = if let Some(path) = &value.file {
            DataSourceKind::File(path.to_path_buf())
        } else {
            DataSourceKind::StdIn
        };
        Self {
            kind,
            format,
            headers: value.headers,
        }
    }
}

pub fn read_csv(
    source: impl io::Read,
    has_headers: bool,
    user_headers: Option<Vec<String>>,
) -> Result<Value> {
    let headless = !has_headers && user_headers.is_none();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(!headless)
        .from_reader(source);
    if let Some(h) = user_headers {
        rdr.set_headers(h.into());
    }
    if headless {
        rdr.into_deserialize::<Vec<String>>()
            .map(|result| {
                result
                    .context("failed to parse csv")
                    .map(|values| Value::Array(values.into_iter().map(Value::String).collect()))
            })
            .collect()
    } else {
        rdr.into_deserialize::<serde_json::Map<String, serde_json::Value>>()
            .map(|result| result.context("failed to parse csv").map(Value::Object))
            .collect()
    }
}

pub fn read_jsonlines(source: impl io::Read) -> Result<Value> {
    let objects: Vec<Value> = BufReader::new(source)
        .lines()
        .map(|line| {
            line.context("failed to read line").and_then(|l| {
                serde_json::from_str::<Value>(&l)
                    .with_context(|| format!("failed to parse json from string:\n\"{}\"", &l))
            })
        })
        .collect::<Result<_>>()?;
    Ok(Value::Array(objects))
}

pub fn read_json(source: impl io::Read) -> Result<Value> {
    serde_json::from_reader(source).context("failed to parse json")
}
