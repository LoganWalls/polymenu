use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;

use crate::io::IOFormat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Command {
    pub command: Vec<String>,
    #[serde(default)]
    pub output_format: IOFormat,
}

impl Command {
    pub fn new(command: Vec<String>, output_format: IOFormat) -> Self {
        if command.is_empty() {
            panic!("script command should have at least one part")
        }
        Self {
            command,
            output_format,
        }
    }

    pub fn call(&self, args: HashMap<String, String>) -> anyhow::Result<Cursor<Vec<u8>>> {
        let output = std::process::Command::new(
            self.command
                .first()
                .expect("script command should have at least one part"),
        )
        .args(self.command.iter().skip(1).map(|a| {
            if a.starts_with("$") {
                args.get(&a.chars().skip(1).collect::<String>())
                    .unwrap_or_else(|| {
                        panic!(
                            "variable {a} was not passed when calling {0:?}",
                            self.command
                        )
                    })
            } else {
                a
            }
        }))
        .output()?;
        Ok(Cursor::new(output.stdout))
    }
}
