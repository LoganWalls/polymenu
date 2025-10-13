use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;

use crate::expansion::shell_expand;
use crate::io::IOFormat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Command {
    pub command: Vec<String>,
    #[serde(default)]
    pub output_format: IOFormat,
}

impl Command {
    pub fn call(&self, args: HashMap<String, String>) -> Result<Cursor<Vec<u8>>> {
        let first = self
            .command
            .first()
            .ok_or_else(|| anyhow!("commands should have at least one part"))?;
        let output = std::process::Command::new(shell_expand(first, &args)?)
            .args(
                self.command
                    .iter()
                    .skip(1)
                    .map(|h| shell_expand(h, &args))
                    .collect::<Result<Vec<_>>>()?,
            )
            .output()?;
        Ok(Cursor::new(output.stdout))
    }
}
