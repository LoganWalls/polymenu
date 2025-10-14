use std::thread::sleep;
use std::time::Duration;

use self::config::{Config, UpdateFromOther};
use self::dev_server::run_dev_server;
use self::expansion::expand_path;
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
    let config_str = cli_opts.config.as_ref().map_or_else(
        || include_str!("../../default-config.toml").to_string(),
        |path| {
            std::fs::read_to_string(expand_path(path).expect("could not expand config path"))
                .unwrap()
        },
    );
    let mut config: Config = toml::from_str(&config_str)?;
    config.update_from_other(cli_opts);

    let event_loop: EventLoop<AppEvent> = EventLoopBuilder::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let shutdown_bridge = ShutdownBridge::new(event_loop_proxy);

    let server = {
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

    run_gui(&config, event_loop, shutdown_bridge.token).await?;
    server.abort();
    Ok(())
}
