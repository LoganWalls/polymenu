use self::config::Config;
use clap::Parser;
use winit::event_loop::EventLoop;

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
    let event_loop = EventLoop::new()?;
    let mut gui = self::gui::GUIApp::default();
    event_loop.run_app(&mut gui)?;
    server.abort();
    Ok(())
}
