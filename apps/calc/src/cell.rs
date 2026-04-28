//! Cell value types for the spreadsheet engine.

use serde::{Deserialize, Serialize};

/// The value stored in a single cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellValue {
    /// An empty, unset cell.
    Empty,
    /// A plain text string.
    Text(String),
    /// A numeric value (f64 covers integers, decimals, dates-as-numbers).
    Number(f64),
    /// A boolean value.
    Boolean(bool),
    /// An error produced during formula evaluation.
    Error(CellError),
}

/// Errors that can appear as a cell's computed value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellError {
    DivisionByZero,
    InvalidReference,
    ParseError,
    CircularReference,
    TypeError,
}

/// A cell address (zero-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellAddress {
    pub col: u32,
    pub row: u32,
}

impl CellAddress {
    pub const fn new(col: u32, row: u32) -> Self {
        Self { col, row }
    }

    /// Convert a column index to a letter label (0 → "A", 25 → "Z", 26 → "AA").
    pub fn col_label(&self) -> String {
        let mut label = String::new();
        let mut c = self.col;
        loop {
            label.insert(0, (b'A' + (c % 26) as u8) as char);
            if c < 26 {
                break;
            }
            c = c / 26 - 1;
        }
        label
    }

    /// Human-readable display like "A1", "B12".
    pub fn display(&self) -> String {
        format!("{}{}", self.col_label(), self.row + 1)
    }
}

/// The data stored for each cell, including the raw input and computed value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// What the user typed (may be a formula starting with `=`).
    pub input: String,
    /// The computed or literal value.
    pub value: CellValue,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            input: String::new(),
            value: CellValue::Empty,
        }
    }

    /// Returns `true` if the cell input is a formula.
    pub fn is_formula(&self) -> bool {
        self.input.starts_with('=')
    }
}
