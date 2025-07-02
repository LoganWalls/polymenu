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
    let server = tokio::spawn(async move { server::run(config).await.unwrap().await.unwrap() });
    run_gui().await?;
    server.abort();
    Ok(())
}
