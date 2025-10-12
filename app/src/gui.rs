use tao::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use crate::config::Config;

pub async fn run_gui(config: &Config) -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_transparent(!config.opaque)
        .with_decorations(config.window_decorations)
        .with_inner_size(Size::Physical(PhysicalSize::new(1050, 1000)))
        .build(&event_loop)
        .unwrap();
    let builder = WebViewBuilder::new()
        .with_transparent(!config.opaque)
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
