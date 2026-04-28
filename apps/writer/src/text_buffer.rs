//! Rope-based text buffer for efficient large-document editing.
//!
//! A "rope" is a balanced tree of string chunks.  Insertions and deletions
//! in the middle of a multi-megabyte document run in O(log n) instead of
//! the O(n) cost of a flat `String`.

/// A single contiguous chunk of UTF-8 text.
#[derive(Debug, Clone)]
struct Chunk {
    text: String,
}

/// A rope-style text buffer optimised for large documents.
///
/// Phase 1 uses a simple gap-buffer; we will swap in a true rope
/// (or integrate `ropey`) once the editing pipeline is wired up.
#[derive(Debug)]
pub struct TextBuffer {
    /// The raw text content (flat for now).
    content: String,
    /// Cursor byte offset into `content`.
    cursor: usize,
}

impl TextBuffer {
    /// Create an empty buffer.
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
        }
    }

    /// Create a buffer pre-loaded with `text`.
    pub fn from_str(text: &str) -> Self {
        Self {
            content: text.to_string(),
            cursor: 0,
        }
    }

    /// Insert `text` at the current cursor position.
    pub fn insert(&mut self, text: &str) {
        self.content.insert_str(self.cursor, text);
        self.cursor += text.len();
    }

    /// Delete `count` bytes behind the cursor (backspace).
    pub fn delete_backward(&mut self, count: usize) {
        let remove = count.min(self.cursor);
        let start = self.cursor - remove;
        self.content.drain(start..self.cursor);
        self.cursor = start;
    }

    /// Move the cursor to an absolute byte offset (clamped).
    pub fn set_cursor(&mut self, offset: usize) {
        self.cursor = offset.min(self.content.len());
    }

    /// Return the full text content as a slice.
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Total length in bytes.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Current cursor byte offset.
    pub fn cursor_offset(&self) -> usize {
        self.cursor
    }
}
