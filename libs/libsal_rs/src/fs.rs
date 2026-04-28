//! Zero-copy file I/O built on memory-mapped files.
//!
//! Documents are never fully buffered into heap memory. Instead we map the
//! backing file directly and let the OS page in only the bytes the viewport
//! actually touches, keeping resident memory well under 20 MB.

use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum FsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// ---------------------------------------------------------------------------
// Memory-mapped file handle
// ---------------------------------------------------------------------------

/// A read-only, zero-copy handle to a file on disk.
///
/// The underlying bytes are memory-mapped; the OS is free to page them in
/// and out as needed so that our RSS stays minimal.
pub struct MappedFile {
    _file: File,
    mmap: Mmap,
}

impl MappedFile {
    /// Open and memory-map `path` for reading.
    pub fn open(path: &Path) -> Result<Self, FsError> {
        let file = File::open(path)?;
        // SAFETY: We hold the File open for the lifetime of the Mmap and
        // treat the mapping as immutable.
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { _file: file, mmap })
    }

    /// Return the mapped bytes (zero-copy).
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Total file length in bytes.
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    /// Whether the mapped region is empty.
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
}
