use crate::callback::Callback;
use polymenu_common::item;
use std::error::Error;
use std::io;
use std::path::PathBuf;

pub enum ItemSource {
    StdIn,
    File(PathBuf),
    Callback(Callback),
}

impl ItemSource {
    pub fn new(cli_args: &polymenu_common::Config) -> Self {
        match (&cli_args.file, &cli_args.callback) {
            (None, None) => Self::StdIn,
            (Some(path), _) => Self::File(path.to_path_buf()),
            (None, Some(args)) => Self::Callback(Callback::new(args.to_vec())),
        }
    }

    pub fn get_items(&mut self, query: &str) -> Result<Vec<item::Item>, Box<dyn Error>> {
        match self {
            Self::StdIn => {
                let source = io::stdin();
                item::parse_items(source)
            }
            Self::File(path) => {
                let source = std::fs::File::open(path)?;
                item::parse_items(source)
            }
            Self::Callback(callback) => callback.call(query),
        }
    }
}
