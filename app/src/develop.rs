use std::future;
use std::path::PathBuf;
use std::pin::pin;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use futures_util::{StreamExt, stream};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use killport::killport::{Killport, KillportOperations};
use killport::signal::KillportSignal;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tower_http::body::Full;
use wry::http::Uri;

use crate::config::Config;
use crate::expansion::{expand_path, shell_expand};

pub async fn run_dev_server(config: &Config, shutdown_token: CancellationToken) -> Result<()> {
    let dev_command = &config.develop_command;
    let app_src = PathBuf::from(expand_path(config.src.as_ref().context(
            "`app_src` must be provided either in your config file or as a CLI argument (neither was provided)"
        )?)?);
    let mut dev_server = tokio::process::Command::new(shell_expand(
        dev_command
            .first()
            .context("`develop_command` should have at least one part")?,
        None,
    )?)
    .args(
        dev_command
            .iter()
            .skip(1)
            .map(|c| shell_expand(c, None))
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

pub async fn ping_dev_server(url: String) -> Result<()> {
    const RETRY_COUNT: u32 = 100;
    const RETRY_TIMEOUT: Duration = Duration::from_secs(10);
    const RETRY_INTERVAL: Duration = Duration::from_millis(100);

    let client: Client<_, Full> = Client::builder(TokioExecutor::new()).build_http();
    let uri = Uri::from_str(&url).context("could not build URI from dev server URL")?;

    let connection_success = stream::iter(0..RETRY_COUNT)
        .then(async |_| {
            let res = client.get(uri.clone()).await;
            if res.is_err() {
                sleep(RETRY_INTERVAL).await;
            }
            res
        })
        .filter(|res| future::ready(res.is_ok()));

    timeout(RETRY_TIMEOUT, pin!(connection_success).next()).await?;
    Ok(())
}

pub async fn compile_app(command: &[String], app_src: &PathBuf) -> Result<()> {
    let mut child = tokio::process::Command::new(shell_expand(
        command
            .first()
            .context("`compile_command` should have at least one part")?,
        None,
    )?)
    .args(
        command
            .iter()
            .skip(1)
            .map(|c| shell_expand(c, None))
            .collect::<Result<Vec<_>>>()?,
    )
    .current_dir(app_src)
    .kill_on_drop(true)
    .spawn()
    .context("problem")?;

    let status = child.wait().await?;
    if let Some(code) = status.code()
        && code == 0
    {
        Ok(())
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
