//! # Rust-Office Writer
//!
//! The word-processor application.  Manages the text-buffer, layout engine,
//! and document model for rich-text editing.

mod app;
mod doc_model;
mod layout;
mod text_buffer;

fn main() {
    env_logger::init();
    log::info!("Rust-Office Writer starting");

    libsal_rs::init();
    app::run();
}
