//! Calc application shell.

use libvcl_rs::window::{AppWindow, WindowConfig};

pub fn run() {
    let config = WindowConfig {
        title: "Rust-Office Calc".into(),
        width: 1400,
        height: 800,
        ..Default::default()
    };

    let (_event_loop, _window) = AppWindow::create(&config);
    log::info!("Calc window created — entering event loop");
}
