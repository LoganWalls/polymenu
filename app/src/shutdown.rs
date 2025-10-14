use tao::event_loop::EventLoopProxy;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Shutdown,
}

pub struct ShutdownBridge {
    pub token: CancellationToken,
}

impl ShutdownBridge {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        let token = CancellationToken::new();
        let t = token.clone();
        tokio::spawn(async move {
            t.cancelled().await;
            let _ = proxy.send_event(AppEvent::Shutdown);
        });
        Self { token }
    }
}
