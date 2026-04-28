//! Application window management wrapping winit.

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};

/// Configuration for creating an application window.
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Rust-Office".into(),
            width: 1280,
            height: 720,
        }
    }
}

/// Wraps a winit Window and its EventLoop for the application.
pub struct AppWindow;

impl AppWindow {
    /// Create a new OS window and event loop from the given configuration.
    pub fn create(config: &WindowConfig) -> (EventLoop<()>, Window) {
        let event_loop = EventLoop::new().expect("failed to create event loop");

        let attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height));

        #[allow(deprecated)]
        let window = event_loop
            .create_window(attrs)
            .expect("failed to create window");

        (event_loop, window)
    }
}
