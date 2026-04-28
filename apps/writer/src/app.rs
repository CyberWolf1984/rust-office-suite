//! Writer application shell — window creation and event loop.

use libvcl_rs::window::{AppWindow, WindowConfig};

/// Launch the Writer window and enter the main event loop.
pub fn run() {
    let config = WindowConfig {
        title: "Rust-Office Writer".into(),
        width: 1280,
        height: 800,
        ..Default::default()
    };

    let (_event_loop, _window) = AppWindow::create(&config);

    log::info!("Writer window created — entering event loop");

    // TODO: wire up the Renderer, text buffer, and event dispatch here.
    // For now we just prove the window can be created.
}
