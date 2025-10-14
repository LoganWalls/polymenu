use std::thread::sleep;
use std::time::Duration;

use self::config::{Config, UpdateFromOther};
use self::dev_server::run_dev_server;
use self::gui::run_gui;
use self::shutdown::{AppEvent, ShutdownBridge};
use clap::Parser;
use tao::event_loop::{EventLoop, EventLoopBuilder};

mod command;
mod config;
mod dev_server;
mod expansion;
mod gui;
mod io;
mod keybinds;
mod server;
mod shutdown;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let shutdown_bridge = ShutdownBridge::new(event_loop_proxy);

    {
        let server_config = config.clone();
        let shutdown_token = shutdown_bridge.token.clone();
        tokio::spawn(async move { server::run(server_config, shutdown_token).await.unwrap() })
    };

    if config.develop {
        let server_config = config.clone();
        let shutdown_token = shutdown_bridge.token.clone();
        tokio::spawn(async move {
            run_dev_server(&server_config, shutdown_token)
                .await
                .unwrap()
        });
        // TODO: find a better solution for letting the dev server start before gui queries it.
        sleep(Duration::from_millis(1000));
    }

    let gui_result = run_gui(&config, event_loop, shutdown_bridge.token.clone()).await;
    if gui_result.is_err() {
        shutdown_bridge.token.cancel();
    }
    gui_result
}
