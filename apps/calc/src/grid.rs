//! Sparse cell grid — only allocates memory for cells that contain data.

use std::collections::HashMap;
use crate::cell::{Cell, CellAddress, CellValue};

/// A sparse grid of cells keyed by `CellAddress`.
///
/// Empty cells are not stored, so a million-row sheet with only 50 populated
/// cells uses essentially zero extra memory.
#[derive(Debug, Clone)]
pub struct Grid {
    cells: HashMap<CellAddress, Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    /// Get the cell at `addr`, or `None` if empty.
    pub fn get(&self, addr: CellAddress) -> Option<&Cell> {
        self.cells.get(&addr)
    }

    /// Get a mutable reference to the cell, inserting an empty one if absent.
    pub fn get_mut(&mut self, addr: CellAddress) -> &mut Cell {
        self.cells.entry(addr).or_insert_with(Cell::empty)
    }

    /// Set the user input for a cell and mark it for recalculation.
    pub fn set_input(&mut self, addr: CellAddress, input: String) {
        let cell = self.get_mut(addr);
        cell.input = input;
        // Value will be computed during the next recalc pass.
        cell.value = CellValue::Empty;
    }

    /// Iterate over all populated cells.
    pub fn iter(&self) -> impl Iterator<Item = (&CellAddress, &Cell)> {
        self.cells.iter()
    }

    /// Number of populated cells.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use calc::grid::Grid;
    /// use calc::cell::CellAddress;
    /// let mut grid = Grid::new();
    /// assert_eq!(grid.populated_count(), 0);
    /// grid.set_input(CellAddress::new(0, 0), "123".into());
    /// assert_eq!(grid.populated_count(), 1);
    /// ```
    pub fn populated_count(&self) -> usize {
        self.cells.len()
    }

    /// Remove a cell (clearing it).
    pub fn clear_cell(&mut self, addr: CellAddress) {
        self.cells.remove(&addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_operations() {
        let mut grid = Grid::new();
        let addr = CellAddress::new(0, 0); // A1
        
        assert!(grid.get(addr).is_none());
        assert_eq!(grid.populated_count(), 0);

        grid.set_input(addr, "42".into());
        assert_eq!(grid.populated_count(), 1);
        assert_eq!(grid.get(addr).unwrap().input, "42");

        grid.clear_cell(addr);
        assert_eq!(grid.populated_count(), 0);
        assert!(grid.get(addr).is_none());
    }
}
