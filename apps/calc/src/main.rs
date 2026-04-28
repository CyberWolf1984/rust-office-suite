//! # Rust-Office Calc
//!
//! The spreadsheet application.  Manages the cell grid, formula parser,
//! dependency graph, and multi-threaded recalculation engine.

mod app;
mod cell;
mod formula;
mod grid;
mod sheet;

fn main() {
    env_logger::init();
    log::info!("Rust-Office Calc starting");

    libsal_rs::init();
    app::run();
}
