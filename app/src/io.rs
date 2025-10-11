use anyhow::Context;
use serde_json::Value;

use crate::command::{Command, IOFormat};
use crate::config::Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

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

    pub fn parse(&self, args: HashMap<String, String>) -> anyhow::Result<Vec<Value>> {
        let mut source: Box<dyn io::Read> = match self.kind.clone() {
            DataSourceKind::StdIn => Box::new(io::stdin()),
            DataSourceKind::File(path) => {
                Box::new(File::open(path).context("failed to open file")?)
            }
            DataSourceKind::Command(callback) => {
                Box::new(callback.call(args).context("failed to execute callback")?)
            }
        };
        match self.format {
            IOFormat::HeadlessCsv => read_csv(
                source,
                false,
                self.headers
                    .clone()
                    .or_else(|| Some(vec!["key".into(), "value".into()])),
            ),
            IOFormat::Csv => read_csv(source, true, self.headers.clone()),
            IOFormat::Json => read_json(source),
            IOFormat::JsonLines => read_jsonlines(source),
            IOFormat::Raw => {
                let mut buf = String::new();
                source
                    .read_to_string(&mut buf)
                    .context("failed to read raw input")?;
                Ok(vec![Value::String(buf)])
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
                    "raw" => IOFormat::Raw,
                    _ => IOFormat::HeadlessCsv,
                }
            } else {
                IOFormat::default()
            }
        });
        let kind = match (&value.file, &value.input_script) {
            (None, None) => DataSourceKind::StdIn,
            (Some(path), _) => DataSourceKind::File(path.to_path_buf()),
            (None, Some(args)) => DataSourceKind::Command(Command::new(args.to_vec(), format)),
        };
        Self {
            kind,
            format,
            headers: value.columns.clone(),
        }
    }
}

pub fn read_csv(
    source: impl io::Read,
    has_headers: bool,
    user_headers: Option<Vec<String>>,
) -> anyhow::Result<Vec<Value>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(source);
    if let Some(h) = user_headers {
        rdr.set_headers(h.into());
    }
    let mut result = Vec::new();
    for data in rdr.into_deserialize() {
        let data: serde_json::Map<String, serde_json::Value> = data?;
        result.push(Value::Object(data));
    }
    Ok(result)
}

pub fn read_jsonlines(source: impl io::Read) -> anyhow::Result<Vec<Value>> {
    BufReader::new(source)
        .lines()
        .map(|line| {
            line.context("failed to read line").and_then(|l| {
                serde_json::from_str(&l)
                    .with_context(|| format!("failed to parse json from string:\n\"{}\"", &l))
            })
        })
        .collect()
}

pub fn read_json(source: impl io::Read) -> anyhow::Result<Vec<Value>> {
    serde_json::from_reader(source).context("failed to parse json")
}
