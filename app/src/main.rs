use self::config::Config;
use self::gui::run_gui;
use clap::Parser;

mod callback;
mod config;
mod gui;
mod item;
mod item_source;
mod keybinds;
mod matcher;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_opts = Config::try_parse()?;
    dbg!(&cli_opts);
    let server = tokio::spawn(async move { server::run(cli_opts).await.unwrap().await.unwrap() });
    let _ = run_gui().await;
    server.abort();
    Ok(())
}
