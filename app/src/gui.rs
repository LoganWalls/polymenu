use anyhow::anyhow;
use tao::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tokio_util::sync::CancellationToken;
use wry::WebViewBuilder;

use crate::config::Config;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Shutdown,
}

#[cfg(target_os = "linux")]
fn is_wayland_window(window: &tao::window::Window) -> bool {
    use gtk::{glib::ObjectExt, traits::WidgetExt};
    use tao::platform::unix::WindowExtUnix;
    window.gtk_window().display().type_().name() == "GdkWaylandDisplay"
}

#[cfg(target_os = "linux")]
fn setup_wayland_layer_shell(
    window: &tao::window::Window,
    vbox: &gtk::Box,
    size: Option<PhysicalSize<u32>>,
) -> anyhow::Result<gtk::ApplicationWindow> {
    use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
    use gtk_layer_shell::LayerShell;
    use tao::platform::unix::WindowExtUnix;

    let gtk_window = window.gtk_window();

    // Hide the original tao window
    gtk_window.hide();

    let app = gtk_window
        .application()
        .ok_or_else(|| anyhow!("GTK application not available"))?;

    // Create a new gtk window for the overlay
    let overlay_window = gtk::ApplicationWindow::new(&app);

    // To prevent the window from being black initially.
    overlay_window.set_app_paintable(true);

    // Move vbox to overlay window
    gtk_window.remove(vbox);
    overlay_window.add(vbox);

    overlay_window.init_layer_shell();
    overlay_window.set_layer(gtk_layer_shell::Layer::Overlay);
    overlay_window.set_keyboard_interactivity(true);

    if let Some(size) = size {
        overlay_window.set_size_request(size.width as i32, size.height as i32);
    }

    overlay_window.show_all();
    Ok(overlay_window)
}

pub fn run_gui(
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
        .with_url(config.gui_target_url());

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let _webview = builder.build(&window)?;

    #[cfg(target_os = "linux")]
    let (_webview, _overlay_window) = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        let webview = builder.build_gtk(vbox)?;
        let overlay_window = is_wayland_window(&window)
            .then(|| setup_wayland_layer_shell(&window, vbox, config.window.size()))
            .transpose()?;
        (webview, overlay_window)
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // Shutdown the server and dev server if they haven't been shutdown already
                shutdown_token.cancel();
            }
            Event::UserEvent(AppEvent::Shutdown) => {
                // Exit the application & close the GUI window
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
