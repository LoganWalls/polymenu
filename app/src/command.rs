use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::expansion::shell_expand;
use crate::io::IOFormat;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";

#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Command {
    pub command: Vec<String>,
    #[serde(default)]
    pub output_format: IOFormat,
}

impl Command {
    pub async fn call(
        &self,
        args: Option<&HashMap<String, String>>,
        stdin_lines: Option<Vec<String>>,
    ) -> Result<Cursor<Vec<u8>>> {
        let first = self
            .command
            .first()
            .ok_or_else(|| anyhow!("commands should have at least one part"))?;
        let mut child = tokio::process::Command::new(shell_expand(first, args)?)
            .args(
                self.command
                    .iter()
                    .skip(1)
                    .map(|h| shell_expand(h, args))
                    .collect::<Result<Vec<_>>>()?,
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(lines) = stdin_lines
            && let Some(mut stdin) = child.stdin.take()
        {
            stdin.write_all(lines.join(LINE_ENDING).as_bytes()).await?;
        }
        let mut output = Vec::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_end(&mut output).await?;
        }

        let status = child.wait().await?;
        if let Some(code) = status.code()
            && code == 0
        {
            Ok(Cursor::new(output))
        } else {
            Err(anyhow!(
                "Non-zero exit status: {}",
                status
                    .code()
                    .map(|i| i.to_string())
                    .unwrap_or("(cannot parse exit code)".to_string())
            ))
        }
    }
}
