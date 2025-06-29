use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
use wry::WebViewBuilder;

#[derive(Default)]
pub struct GUIApp {
    window: Option<Window>,
    webview: Option<wry::WebView>,
}

impl ApplicationHandler for GUIApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_transparent(true))
            .unwrap();
        let builder = WebViewBuilder::new().with_url("http://localhost:5173");

        #[cfg(not(target_os = "linux"))]
        let webview = builder.build(&window).unwrap();
        #[cfg(target_os = "linux")]
        let webview = builder.build_gtk(window.gtk_window()).unwrap();

        self.window = Some(window);
        self.webview = Some(webview);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}
