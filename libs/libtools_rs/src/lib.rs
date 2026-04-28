//! # libtools_rs — Core Toolkit Types
//!
//! Provides fundamental value types used across every layer of Rust-Office:
//! colours, rectangles, measurement units, and SIMD-accelerated math via glam.

pub mod color;
pub mod rect;
pub mod units;

pub use color::Color;
pub use rect::Rect;
