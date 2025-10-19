use self::config::{Config, UpdateFromOther};
use self::develop::{compile_app, ping_dev_server, run_dev_server};
use self::gui::{AppEvent, run_gui};
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use tao::event_loop::{EventLoop, EventLoopBuilder};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

mod command;
mod config;
mod develop;
mod expansion;
mod gui;
mod io;
mod keybinds;
mod server;

fn main() -> Result<()> {
    let cli_opts = Config::try_parse()?;
    let mut config = Config::from_file(
        &cli_opts
            .config
            .to_owned()
            .unwrap_or_else(Config::default_path),
    )?;
    config.update_from_other(cli_opts);
    config = config.apply_cli_overrides()?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .context("failed to initialize tokio")?;

    if config.compile {
        rt.block_on(compile_app(
            &config.compile_command,
            &config.src.expect("no `src` provided"),
        ))
        .context("failed to run compile command")?;
        return Ok(());
    }

    let event_loop: EventLoop<AppEvent> = EventLoopBuilder::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let shutdown_token = CancellationToken::new();
    let server: JoinHandle<Result<()>> = {
        let server_config = config.clone();
        let shutdown_token = shutdown_token.clone();
        rt.spawn(async move {
            server::run(server_config, shutdown_token)
                .await
                .context("problem with server")
        })
    };

    let dev_server: JoinHandle<Result<()>> = if config.develop {
        let server_config = config.clone();
        let shutdown_token = shutdown_token.clone();
        let dev_server = rt.spawn(async move {
            run_dev_server(&server_config, shutdown_token)
                .await
                .context("problem with dev server")
        });

        // Wait for the dev server to boot up before we continue
        // (otherwise the window might open and show a 404)
        rt.block_on(ping_dev_server(config.gui_target_url()))?;

        dev_server
    } else {
        rt.spawn(async { Ok(()) })
    };

    rt.spawn(async move {
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

    let gui_result = run_gui(&config, event_loop, shutdown_token.clone());
    if gui_result.is_err() {
        shutdown_token.cancel();
    }
    gui_result
}
