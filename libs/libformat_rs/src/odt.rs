//! Parser stub for OpenDocument Text (.odt) files.
//!
//! An ODT file is a ZIP archive containing `content.xml`, `styles.xml`,
//! and `meta.xml`.  We stream-parse each XML file using SAX events.

use crate::FormatError;
use std::path::Path;

/// Placeholder for a parsed ODT document structure.
pub struct OdtDocument {
    pub body_text: String,
}

/// Open and stream-parse an .odt file.
pub fn parse_odt(_path: &Path) -> Result<OdtDocument, FormatError> {
    // TODO: Implement SAX-style parsing of content.xml inside the ZIP.
    log::info!("libformat_rs: ODT parser invoked (stub)");
    Ok(OdtDocument {
        body_text: String::new(),
    })
}
