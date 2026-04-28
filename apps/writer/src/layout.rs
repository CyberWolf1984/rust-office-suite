//! Text layout engine for Writer.
//!
//! Converts a sequence of styled paragraphs into positioned glyphs ready
//! for GPU rendering.  The layout runs on a background thread so the UI
//! thread is never blocked during pagination.

use crate::doc_model::Paragraph;
use libtools_rs::Rect;

/// A laid-out line of text with its bounding box on the page.
#[derive(Debug, Clone)]
pub struct LayoutLine {
    /// Bounding rectangle in page coordinates (twips).
    pub bounds: Rect,
    /// Index of the paragraph this line belongs to.
    pub paragraph_index: usize,
    /// Byte range within the paragraph's flattened text.
    pub byte_range: std::ops::Range<usize>,
}

/// Page geometry constants (A4 portrait in twips).
pub struct PageGeometry {
    pub width: f64,
    pub height: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
}

impl Default for PageGeometry {
    fn default() -> Self {
        use libtools_rs::units::cm_to_twips;
        Self {
            width: cm_to_twips(21.0),   // A4
            height: cm_to_twips(29.7),  // A4
            margin_top: cm_to_twips(2.54),
            margin_bottom: cm_to_twips(2.54),
            margin_left: cm_to_twips(2.54),
            margin_right: cm_to_twips(2.54),
        }
    }
}

/// Perform a full layout pass over the given paragraphs.
///
/// Returns a list of positioned lines.  This is intentionally a pure
/// function so it can be run on a `rayon` thread pool.
pub fn layout_paragraphs(
    paragraphs: &[Paragraph],
    _page: &PageGeometry,
) -> Vec<LayoutLine> {
    // TODO: implement real line-breaking, font shaping, and pagination.
    let mut lines = Vec::new();
    for (i, _para) in paragraphs.iter().enumerate() {
        lines.push(LayoutLine {
            bounds: Rect::new(0.0, (i as f32) * 20.0, 500.0, 16.0),
            paragraph_index: i,
            byte_range: 0..0,
        });
    }
    lines
}
