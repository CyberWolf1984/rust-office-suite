//! Parser stub for Office Open XML Spreadsheet (.xlsx) files.

use crate::FormatError;
use std::path::Path;

pub struct XlsxSpreadsheet {
    pub sheet_names: Vec<String>,
}

pub fn parse_xlsx(_path: &Path) -> Result<XlsxSpreadsheet, FormatError> {
    log::info!("libformat_rs: XLSX parser invoked (stub)");
    Ok(XlsxSpreadsheet {
        sheet_names: vec!["Sheet1".to_string()],
    })
}
