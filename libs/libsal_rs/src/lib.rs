//! # libsal_rs — System Abstraction Layer
//!
//! Provides zero-copy file I/O via memory-mapped files, async runtime
//! bootstrapping, and cross-platform path resolution. This is the lowest
//! layer in the Rust-Office stack — everything else depends on it.

pub mod fs;
pub mod platform;

/// Initialise the SAL subsystem (logging, async runtime handle, etc.).
pub fn init() {
    log::info!("libsal_rs: System Abstraction Layer initialised");
}
