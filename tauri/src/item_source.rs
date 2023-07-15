use crate::callback::Callback;
use polymenu_common::item::{Item, ItemData};
use polymenu_common::ItemFormat;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub enum InputSource {
    StdIn,
    File(PathBuf),
    Callback(Callback),
}

pub struct ItemSource {
    input: InputSource,
    format: ItemFormat,
}

impl ItemSource {
    pub fn new(cli_args: &polymenu_common::Config) -> Self {
        let format = cli_args.format.unwrap_or_else(|| {
            if let Some(extension) = cli_args
                .file
                .as_deref()
                .and_then(|p| p.extension())
                .and_then(|e| e.to_str())
            {
                match extension {
                    "csv" => ItemFormat::Csv,
                    "json" => ItemFormat::Json,
                    _ => ItemFormat::default(),
                }
            } else {
                ItemFormat::default()
            }
        });
        let input = match (&cli_args.file, &cli_args.callback) {
            (None, None) => InputSource::StdIn,
            (Some(path), _) => InputSource::File(path.to_path_buf()),
            (None, Some(args)) => InputSource::Callback(Callback::new(args.to_vec())),
        };
        Self { input, format }
    }

    pub fn get_items(&mut self, query: &str) -> Result<Vec<Item>, Box<dyn Error>> {
        let source: Box<dyn io::Read> = match &mut self.input {
            InputSource::StdIn => Box::new(io::stdin()),
            InputSource::File(path) => Box::new(File::open(path)?),
            InputSource::Callback(callback) => Box::new(callback.call(query)?),
        };
        match self.format {
            ItemFormat::HeadlessCsv => read_csv(source, false),
            ItemFormat::Csv => read_csv(source, true),
            ItemFormat::Json => read_json(source),
        }
    }
}

pub fn read_csv(source: impl io::Read, has_headers: bool) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(source);
    let mut result = Vec::new();
    for (i, data) in rdr.deserialize::<ItemData>().enumerate() {
        result.push(Item::new(i, data?));
    }
    Ok(result)
}

pub fn read_json(source: impl io::Read) -> Result<Vec<Item>, Box<dyn Error>> {
    Ok(BufReader::new(source)
        .lines()
        .map(|line| line.and_then(|l| Ok(serde_json::from_str(&l)?)))
        .enumerate()
        .map(|(i, item_data)| item_data.map(|d| Item::new(i, d)))
        .collect::<Result<Vec<_>, _>>()?)
}
