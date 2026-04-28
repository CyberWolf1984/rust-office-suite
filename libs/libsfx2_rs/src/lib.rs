//! # libsfx2_rs — Core Application Framework
//!
//! Manages the document lifecycle (new / open / save / close), undo/redo
//! history, the command dispatch system, and application state.

pub mod command;
pub mod document;
pub mod undo;

pub use document::{Document, DocumentState};
