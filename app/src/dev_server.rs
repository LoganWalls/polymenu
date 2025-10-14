use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::expansion::expand_path;

pub async fn run_dev_server(config: &Config, shutdown_token: CancellationToken) -> Result<()> {
    let dev_command = &config.develop_command;
    let app_src = PathBuf::from(expand_path(config.app_src.as_ref().context(
            "`app_src` must be provided either in your config file or as a CLI argument (neither was provided)"
        )?)?);
    let mut dev_server = tokio::process::Command::new(
        dev_command
            .first()
            .context("`develop_command` should have at least one part")?,
    )
    .current_dir(app_src)
    .env("API_PORT", &config.port)
    .env("DEV_SERVER_PORT", &config.dev_server_port)
    .args(dev_command.iter().skip(1))
    .kill_on_drop(true)
    .spawn()
    .context("Problem starting dev server")?;
    tokio::select! {
        status = dev_server.wait() => {
            let status = status.context("Problem getting exit status")?;
            println!("Dev server exited with status: {status}")
        }
        _ = shutdown_token.cancelled() => {
            println!("Shutting down dev server...");
            let _ = dev_server.kill().await;
            let _ = dev_server.wait().await;
        }
    }
    Ok(())
}
