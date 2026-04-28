//! Parser stub for OpenDocument Spreadsheet (.ods) files.

use crate::FormatError;
use std::path::Path;

pub struct OdsSpreadsheet {
    pub sheet_names: Vec<String>,
}

pub fn parse_ods(_path: &Path) -> Result<OdsSpreadsheet, FormatError> {
    log::info!("libformat_rs: ODS parser invoked (stub)");
    Ok(OdsSpreadsheet {
        sheet_names: vec!["Sheet1".to_string()],
    })
}
