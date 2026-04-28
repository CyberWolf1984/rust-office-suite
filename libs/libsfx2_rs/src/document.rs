//! Document lifecycle and state management.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// The current state of a document in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentState {
    /// A brand-new, unsaved document.
    New,
    /// Loaded from disk and unmodified.
    Clean,
    /// Contains unsaved modifications.
    Dirty,
    /// Currently being saved to disk.
    Saving,
}

/// Metadata shared by every document type (Writer, Calc, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    /// Display title shown in the title bar.
    pub title: String,
    /// On-disk path, if the document has been saved.
    pub path: Option<PathBuf>,
    /// Author field (from ODF / OOXML metadata).
    pub author: Option<String>,
}

/// Trait that every concrete document type must implement.
pub trait Document {
    /// Return shared metadata.
    fn meta(&self) -> &DocumentMeta;

    /// Current lifecycle state.
    fn state(&self) -> DocumentState;

    /// Mark the document as modified.
    fn mark_dirty(&mut self);

    /// Mark the document as clean (just saved).
    fn mark_clean(&mut self);

    /// Serialise the document to bytes in its native format.
    fn save_to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
