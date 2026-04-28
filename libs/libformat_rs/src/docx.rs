//! Parser stub for Office Open XML Document (.docx) files.

use crate::FormatError;
use std::path::Path;

pub struct DocxDocument {
    pub body_text: String,
}

pub fn parse_docx(_path: &Path) -> Result<DocxDocument, FormatError> {
    log::info!("libformat_rs: DOCX parser invoked (stub)");
    Ok(DocxDocument {
        body_text: String::new(),
    })
}
