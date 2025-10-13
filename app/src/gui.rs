use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use crate::config::Config;

pub async fn run_gui(config: &Config) -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
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

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
