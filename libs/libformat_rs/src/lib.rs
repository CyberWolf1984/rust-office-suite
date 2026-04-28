//! # libformat_rs — Document Format Parsers
//!
//! SAX-style (event-driven) parsers for ODF and OOXML formats.
//! All parsers stream from memory-mapped buffers — we never materialise
//! an entire XML tree in RAM.

pub mod csv;
pub mod docx;
pub mod odt;
pub mod ods;
pub mod xlsx;
pub mod plaintext;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Unsupported format: {0}")]
    Unsupported(String),
}

/// Detect the format of a file from its extension and return a human name.
pub fn detect_format(path: &std::path::Path) -> Result<&'static str, FormatError> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("odt") => Ok("OpenDocument Text"),
        Some("ods") => Ok("OpenDocument Spreadsheet"),
        Some("docx") => Ok("Office Open XML Document"),
        Some("xlsx") => Ok("Office Open XML Spreadsheet"),
        Some("csv") => Ok("Comma-Separated Values"),
        Some("txt") => Ok("Plain Text"),
        Some(ext) => Err(FormatError::Unsupported(ext.to_string())),
        None => Err(FormatError::Unsupported("(no extension)".to_string())),
    }
}
