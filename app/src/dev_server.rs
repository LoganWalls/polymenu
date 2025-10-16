use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result};
use killport::killport::{Killport, KillportOperations};
use killport::signal::KillportSignal;
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::expansion::{expand_path, shell_expand};

pub async fn run_dev_server(config: &Config, shutdown_token: CancellationToken) -> Result<()> {
    let dev_command = &config.develop_command;
    let app_src = PathBuf::from(expand_path(config.app_src.as_ref().context(
            "`app_src` must be provided either in your config file or as a CLI argument (neither was provided)"
        )?)?);
    let args = HashMap::new();
    let mut dev_server = tokio::process::Command::new(shell_expand(
        dev_command
            .first()
            .context("`develop_command` should have at least one part")?,
        &args,
    )?)
    .args(
        dev_command
            .iter()
            .map(|c| shell_expand(c, &args))
            .collect::<Result<Vec<_>>>()?,
    )
    .current_dir(app_src)
    .env("API_PORT", &config.port)
    .env("DEV_SERVER_PORT", &config.dev_server_port)
    .kill_on_drop(true)
    .spawn()
    .context("Problem starting dev server")?;

    let kp = Killport {};
    let dev_port_int: u16 = config.dev_server_port.parse()?;
    let result: Result<()>;
    tokio::select! {
        status = dev_server.wait() => {
            let status = status.context("Problem getting exit status")?;
            println!("Dev server exited with status: {status}");
            result = Ok(());
        }
        _ = shutdown_token.cancelled() => {
            // We need to kill the dev server using killport because killing with `.kill().await`
            // only kills the parent dev server process and the child processes become zombies (at
            // least, for Vite).
            result = kp.kill_service_by_port(
                dev_port_int,
                KillportSignal::from_str("SIGKILL").expect("wrong signal vro"),
                killport::cli::Mode::Process,
                false,
            ).map(|_| ()).context("Problem killing dev server process");
        }
    }
    result
}
