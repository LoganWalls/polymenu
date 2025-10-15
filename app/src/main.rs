use std::thread::sleep;
use std::time::Duration;

use self::config::{Config, UpdateFromOther};
use self::dev_server::run_dev_server;
use self::gui::{AppEvent, run_gui};
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use tao::event_loop::{EventLoop, EventLoopBuilder};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

mod command;
mod config;
mod dev_server;
mod expansion;
mod gui;
mod io;
mod keybinds;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_opts = Config::try_parse()?;
    let mut config = Config::from_file(
        &cli_opts
            .config
            .to_owned()
            .unwrap_or_else(Config::default_path),
    )?;
    config.update_from_other(cli_opts);

    let event_loop: EventLoop<AppEvent> = EventLoopBuilder::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let shutdown_token = CancellationToken::new();

    let server: JoinHandle<Result<()>> = {
        let server_config = config.clone();
        let shutdown_token = shutdown_token.clone();
        tokio::spawn(async move {
            server::run(server_config, shutdown_token)
                .await
                .context("problem with server")
        })
    };

    let dev_server: JoinHandle<Result<()>> = if config.develop {
        let server_config = config.clone();
        let shutdown_token = shutdown_token.clone();
        let dev_server = tokio::spawn(async move {
            run_dev_server(&server_config, shutdown_token)
                .await
                .context("problem with dev server")
        });
        // TODO: find a better solution for letting the dev server start before gui queries it.
        sleep(Duration::from_millis(1000));

        dev_server
    } else {
        tokio::spawn(async { Ok(()) })
    };

    tokio::spawn(async move {
        // Await both JoinHandles. This guarantees both tasks have actually exited.
        let results = tokio::try_join!(server, dev_server)
            .map_err(|e| anyhow!("join error: {e}"))
            .unwrap();
        for r in [results.0, results.1] {
            if let Err(e) = r {
                eprintln!("{e}")
            }
        }
        // Sending this event will trigger `std::process::exit`, so nothing can be printed
        // or cleaned up after sending it.
        let _ = event_loop_proxy.send_event(AppEvent::Shutdown);
    });

    let gui_result = run_gui(&config, event_loop, shutdown_token.clone()).await;
    if gui_result.is_err() {
        shutdown_token.cancel();
    }
    gui_result
}
