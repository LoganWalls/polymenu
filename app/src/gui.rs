use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tokio_util::sync::CancellationToken;
use wry::WebViewBuilder;

use crate::{config::Config, shutdown::AppEvent};

pub async fn run_gui(
    config: &Config,
    event_loop: EventLoop<AppEvent>,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    let mut window = WindowBuilder::new()
        .with_transparent(!config.window.opaque)
        .with_decorations(config.window.decorations)
        .with_focused(!config.window.no_focus);
    if let Some(size) = config.window.size() {
        window = window.with_inner_size(size);
    }
    if let Some(position) = config.window.position() {
        window = window.with_position(position);
    }
    let window = window.build(&event_loop).unwrap();
    let builder = WebViewBuilder::new()
        .with_transparent(!config.window.opaque)
        .with_devtools(true)
        .with_url(format!(
            "http://localhost:{}",
            if config.develop {
                &config.dev_server_port
            } else {
                &config.port
            }
        ));

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let _webview = builder.build(&window)?;

    #[cfg(target_os = "linux")]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::UserEvent(AppEvent::Shutdown) => {
                // Shutdown the server and dev server if they haven't been shutdown already
                shutdown_token.cancel();
                // Exit the application & close the GUI window
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
