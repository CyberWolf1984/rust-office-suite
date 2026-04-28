//! CSV parser for Calc.

use crate::FormatError;
use std::path::Path;

pub struct CsvData {
    pub rows: Vec<Vec<String>>,
}

pub fn parse_csv(path: &Path) -> Result<CsvData, FormatError> {
    let content = std::fs::read_to_string(path)?;
    let rows: Vec<Vec<String>> = content
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();
    Ok(CsvData { rows })
}
