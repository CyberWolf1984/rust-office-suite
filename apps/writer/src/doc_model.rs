//! Writer document model.
//!
//! A `WriterDoc` is a tree of paragraphs, each containing a sequence of
//! styled text runs.  This model maps closely to the ODF `<text:p>` /
//! `<text:span>` structure for lossless round-tripping.

use libsfx2_rs::document::{Document, DocumentMeta, DocumentState};
use libtools_rs::Color;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Inline formatting
// ---------------------------------------------------------------------------

/// Font style applied to a contiguous run of text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharFormat {
    pub font_family: String,
    pub font_size_pt: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Color,
}

impl Default for CharFormat {
    fn default() -> Self {
        Self {
            font_family: "Inter".into(),
            font_size_pt: 12.0,
            bold: false,
            italic: false,
            underline: false,
            color: Color::BLACK,
        }
    }
}

// ---------------------------------------------------------------------------
// Text run / paragraph
// ---------------------------------------------------------------------------

/// A contiguous run of text sharing the same character formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub format: CharFormat,
}

/// A single paragraph — one or more `TextRun`s followed by a newline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub runs: Vec<TextRun>,
}

impl Paragraph {
    pub fn plain(text: &str) -> Self {
        Self {
            runs: vec![TextRun {
                text: text.to_string(),
                format: CharFormat::default(),
            }],
        }
    }
}

// ---------------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------------

/// The Writer document model: an ordered list of paragraphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterDoc {
    pub meta: DocumentMeta,
    pub state: DocumentState,
    pub paragraphs: Vec<Paragraph>,
}

impl WriterDoc {
    pub fn new_blank() -> Self {
        Self {
            meta: DocumentMeta {
                title: "Untitled".into(),
                path: None,
                author: None,
            },
            state: DocumentState::New,
            paragraphs: vec![Paragraph::plain("")],
        }
    }
}

impl Document for WriterDoc {
    fn meta(&self) -> &DocumentMeta {
        &self.meta
    }
    fn state(&self) -> DocumentState {
        self.state
    }
    fn mark_dirty(&mut self) {
        self.state = DocumentState::Dirty;
    }
    fn mark_clean(&mut self) {
        self.state = DocumentState::Clean;
    }
    fn save_to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // TODO: serialize to ODF XML inside a ZIP archive.
        // For now, emit the raw paragraph text as UTF-8.
        let text: String = self.paragraphs.iter()
            .flat_map(|p| p.runs.iter().map(|r| r.text.as_str()))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(text.into_bytes())
    }
}
