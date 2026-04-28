//! Sheet model — a named grid with metadata.

use crate::grid::Grid;
use libsfx2_rs::document::{Document, DocumentMeta, DocumentState};
use serde::{Deserialize, Serialize};

/// A single named sheet inside a spreadsheet workbook.
#[derive(Debug, Clone)]
pub struct Sheet {
    pub name: String,
    pub grid: Grid,
}

impl Sheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            grid: Grid::new(),
        }
    }
}

/// A Calc workbook — one or more named sheets.
#[derive(Debug)]
pub struct CalcDoc {
    pub meta: DocumentMeta,
    pub state: DocumentState,
    pub sheets: Vec<Sheet>,
    pub active_sheet: usize,
}

impl CalcDoc {
    pub fn new_blank() -> Self {
        Self {
            meta: DocumentMeta {
                title: "Untitled".into(),
                path: None,
                author: None,
            },
            state: DocumentState::New,
            sheets: vec![Sheet::new("Sheet1")],
            active_sheet: 0,
        }
    }
}

impl Document for CalcDoc {
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
        // TODO: Serialize to ODS XML inside ZIP.
        Ok(Vec::new())
    }
}
