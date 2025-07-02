use serde_json::Map;

use crate::command::{Command, IOFormat};
use crate::config::Config;
use crate::item::Item;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub enum InputSource {
    StdIn,
    File(PathBuf),
    Callback(Command),
}

#[derive(Debug)]
pub struct ItemSource {
    input: InputSource,
    format: IOFormat,
    headers: Option<Vec<String>>,
}

impl ItemSource {
    pub fn new(cli_args: &Config) -> Self {
        let format = cli_args.format.unwrap_or_else(|| {
            if let Some(extension) = cli_args
                .file
                .as_deref()
                .and_then(|p| p.extension())
                .and_then(|e| e.to_str())
            {
                match extension {
                    "csv" => IOFormat::Csv,
                    "json" => IOFormat::Json,
                    "raw" => IOFormat::Raw,
                    _ => IOFormat::HeadlessCsv,
                }
            } else {
                IOFormat::default()
            }
        });
        let input = match (&cli_args.file, &cli_args.input_script) {
            (None, None) => InputSource::StdIn,
            (Some(path), _) => InputSource::File(path.to_path_buf()),
            (None, Some(args)) => InputSource::Callback(Command::new(args.to_vec(), format)),
        };
        Self {
            input,
            format,
            headers: cli_args.columns.clone(),
        }
    }

    pub fn get_items(&mut self, args: HashMap<String, String>) -> anyhow::Result<Vec<Item>> {
        let mut source: Box<dyn io::Read> = match &mut self.input {
            InputSource::StdIn => Box::new(io::stdin()),
            InputSource::File(path) => Box::new(File::open(path)?),
            InputSource::Callback(callback) => Box::new(callback.call(args)?),
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
            IOFormat::Raw => {
                let mut buf = String::new();
                source.read_to_string(&mut buf);
                Ok(vec![Item::new(0, buf, Map::new())])
            }
        }
    }
}

pub fn read_csv(
    source: impl io::Read,
    has_headers: bool,
    user_headers: Option<Vec<String>>,
) -> anyhow::Result<Vec<Item>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(source);
    if let Some(h) = user_headers {
        rdr.set_headers(h.into());
    }
    let mut result = Vec::new();
    for (i, data) in rdr.into_deserialize().enumerate() {
        let data: serde_json::Map<String, serde_json::Value> = data?;
        result.push(Item::try_from_json(i, serde_json::Value::Object(data))?);
    }
    Ok(result)
}

pub fn read_json(source: impl io::Read) -> anyhow::Result<Vec<Item>> {
    BufReader::new(source)
        .lines()
        .map(|line| line.and_then(|l| Ok(serde_json::from_str(&l)?)))
        .enumerate()
        .map(|(i, item_data)| Item::try_from_json(i, item_data?))
        .collect()
}
