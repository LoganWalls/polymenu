use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use self::config::{Config, UpdateFromOther};
use self::gui::run_gui;
use clap::Parser;

mod command;
mod config;
mod gui;
mod io;
mod keybinds;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_opts = Config::try_parse()?;
    let config_str = &cli_opts.config.as_ref().map_or_else(
        || include_str!("../../default-config.toml").to_string(),
        |path| std::fs::read_to_string(path).unwrap(),
    );
    let mut config: Config = toml::from_str(config_str)?;
    config.update_from_other(cli_opts);

    let server = {
        let server_config = config.clone();
        tokio::spawn(async move { server::run(server_config).await.unwrap().await.unwrap() })
    };

    let mut dev_server = None;
    if config.develop {
        let dev_command = &config.develop_command;
        dev_server = Some(
            tokio::process::Command::new(
                dev_command
                    .first()
                    .expect("develop command should have at least one part"),
            )
            .current_dir(PathBuf::from_str("web/")?)
            .env("API_PORT", &config.port)
            .env("DEV_SERVER_PORT", &config.dev_server_port)
            .args(dev_command.iter().skip(1))
            .kill_on_drop(true)
            .spawn()
            .unwrap(),
        );
        // TODO: find a better solution for letting the dev server start before gui queries it.
        sleep(Duration::from_millis(500));
    }

    run_gui(&config).await?;
    server.abort();
    if let Some(s) = &mut dev_server {
        s.kill().await?
    }
    Ok(())
}
